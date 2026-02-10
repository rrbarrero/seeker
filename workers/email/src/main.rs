use anyhow::Context;
use serde::{Deserialize, Serialize};
use sqlx::{
    Pool, Postgres,
    postgres::{PgListener, PgNotification, PgPoolOptions},
};
use std::env;
use std::sync::Arc;
use tokio::time::{Duration, sleep};
use tracing::{error, info};
use uuid::Uuid;

#[async_trait::async_trait]
pub trait EmailSender: Send + Sync {
    async fn send(&self, to: &str, subject: &str, body: &str) -> anyhow::Result<()>;
}

pub struct StdoutEmailSender;

#[async_trait::async_trait]
impl EmailSender for StdoutEmailSender {
    async fn send(&self, to: &str, subject: &str, body: &str) -> anyhow::Result<()> {
        println!("--------------------------------------------------");
        println!("📧 SENDING EMAIL");
        println!("To: {}", to);
        println!("Subject: {}", subject);
        println!("Body: {}", body);
        println!("--------------------------------------------------");
        sleep(Duration::from_millis(500)).await;
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct NotificationPayload {
    id: Uuid,
}

#[derive(Debug, sqlx::FromRow)]
struct EmailJob {
    id: Uuid,
    payload: sqlx::types::Json<EmailPayload>,
    #[allow(dead_code)]
    processed: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct EmailPayload {
    to: String,
    subject: String,
    body: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    info!("Starting Email Worker...");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .context("Failed to connect to Postgres")?;

    info!("Connected to database.");

    let email_sender = Arc::new(StdoutEmailSender);

    if let Err(e) = process_pending_jobs(&pool, email_sender.clone()).await {
        error!("Error processing pending jobs: {:?}", e);
    }

    listen_for_jobs(pool, email_sender).await
}

async fn process_pending_jobs(
    pool: &Pool<Postgres>,
    sender: Arc<dyn EmailSender>,
) -> anyhow::Result<()> {
    info!("Checking for pending jobs...");

    let job_ids: Vec<Uuid> =
        sqlx::query_scalar("SELECT id FROM email_queue WHERE processed = false")
            .fetch_all(pool)
            .await?;

    if job_ids.is_empty() {
        info!("No pending jobs found.");
    } else {
        info!("Found {} potentially pending jobs.", job_ids.len());
        for id in job_ids {
            if let Err(e) = process_job_transactional(pool, id, sender.clone()).await {
                error!("Failed to process pending job {}: {:?}", id, e);
            }
        }
    }

    Ok(())
}

async fn listen_for_jobs(pool: Pool<Postgres>, sender: Arc<dyn EmailSender>) -> anyhow::Result<()> {
    let mut listener = PgListener::connect_with(&pool).await?;
    listener.listen("email_queue").await?;

    info!("Listening for notifications on 'email_queue'...");

    loop {
        let notification = match listener.recv().await {
            Ok(n) => n,
            Err(e) => {
                error!("Error receiving notification: {:?}", e);
                sleep(Duration::from_secs(5)).await;
                continue;
            }
        };

        if let Err(e) = handle_notification(&pool, notification, sender.clone()).await {
            error!("Error handling notification: {:?}", e);
        }
    }
}

async fn handle_notification(
    pool: &Pool<Postgres>,
    notification: PgNotification,
    sender: Arc<dyn EmailSender>,
) -> anyhow::Result<()> {
    let payload_str = notification.payload();
    let payload = serde_json::from_str::<NotificationPayload>(payload_str).context(format!(
        "Failed to parse notification payload: '{}'",
        payload_str
    ))?;

    info!("Received notification for job: {}", payload.id);

    process_job_transactional(pool, payload.id, sender).await?;

    Ok(())
}

async fn process_job_transactional(
    pool: &Pool<Postgres>,
    job_id: Uuid,
    sender: Arc<dyn EmailSender>,
) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    let job_opt = sqlx::query_as::<_, EmailJob>(
        "SELECT id, payload, processed FROM email_queue WHERE id = $1 AND processed = false FOR UPDATE SKIP LOCKED"
    )
    .bind(job_id)
    .fetch_optional(&mut *tx)
    .await?;

    if let Some(job) = job_opt {
        info!("Processing job {}", job.id);
        let payload = job.payload.0;

        match sender
            .send(&payload.to, &payload.subject, &payload.body)
            .await
        {
            Ok(_) => {
                sqlx::query(
                    "UPDATE email_queue SET processed = true, processed_at = NOW() WHERE id = $1",
                )
                .bind(job.id)
                .execute(&mut *tx)
                .await?;

                tx.commit().await?;
                info!("Job {} completed successfully.", job.id);
            }
            Err(e) => {
                error!("Failed to send email: {:?}", e);
                return Err(e);
            }
        }
    }

    Ok(())
}
