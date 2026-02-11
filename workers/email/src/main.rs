use anyhow::Context;
use opentelemetry::KeyValue;
use opentelemetry::trace::TracerProvider;
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{Resource, logs::SdkLoggerProvider, trace::SdkTracerProvider};
use opentelemetry_semantic_conventions::resource::SERVICE_NAME;
use serde::{Deserialize, Serialize};
use sqlx::{
    Pool, Postgres,
    postgres::{PgListener, PgNotification, PgPoolOptions},
};
use std::env;
use std::sync::Arc;
use tokio::time::{Duration, sleep};
use tracing::{error, info};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
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
    user_id: Option<Uuid>,
    trace_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct EmailPayload {
    to: String,
    subject: String,
    body: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let observability_enabled = env::var("OBS_ENABLED")
        .unwrap_or_else(|_| "false".to_string())
        .to_lowercase()
        .as_str()
        == "true";
    let otlp_endpoint = env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
        .unwrap_or_else(|_| "http://localhost:4317".to_string());
    let service_name = env::var("SERVICE_NAME").unwrap_or_else(|_| "email-worker".to_string());

    let _observability = if observability_enabled {
        match init_observability(&service_name, &otlp_endpoint) {
            Ok(obs) => Some(obs),
            Err(err) => {
                eprintln!("Observability disabled: {err}");
                tracing_subscriber::fmt::init();
                None
            }
        }
    } else {
        tracing_subscriber::fmt::init();
        None
    };
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

struct Observability {
    _logger_provider: SdkLoggerProvider,
    _tracer_provider: SdkTracerProvider,
}

fn init_observability(service_name: &str, otlp_endpoint: &str) -> anyhow::Result<Observability> {
    let resource = Resource::builder()
        .with_attributes(vec![KeyValue::new(SERVICE_NAME, service_name.to_string())])
        .build();

    let trace_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint(otlp_endpoint)
        .build()
        .context("OTLP trace exporter error")?;

    let tracer_provider = SdkTracerProvider::builder()
        .with_resource(resource.clone())
        .with_batch_exporter(trace_exporter)
        .build();

    let static_service_name: &'static str = Box::leak(service_name.to_string().into_boxed_str());
    let tracer = tracer_provider.tracer(static_service_name);

    let log_exporter = opentelemetry_otlp::LogExporter::builder()
        .with_tonic()
        .with_endpoint(otlp_endpoint)
        .build()
        .context("OTLP log exporter error")?;

    let logger_provider = SdkLoggerProvider::builder()
        .with_resource(resource)
        .with_batch_exporter(log_exporter)
        .build();

    let otel_logs_layer = OpenTelemetryTracingBridge::new(&logger_provider);
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let fmt_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_current_span(true)
        .with_span_list(true);

    let otel_trace_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .with(otel_trace_layer)
        .with(otel_logs_layer)
        .init();

    Ok(Observability {
        _logger_provider: logger_provider,
        _tracer_provider: tracer_provider,
    })
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
        "SELECT id, payload, processed, user_id, trace_id FROM email_queue WHERE id = $1 AND processed = false FOR UPDATE SKIP LOCKED"
    )
    .bind(job_id)
    .fetch_optional(&mut *tx)
    .await?;

    if let Some(job) = job_opt {
        let trace_id = job.trace_id.as_deref().unwrap_or("-");
        let user_id = job
            .user_id
            .map(|id| id.to_string())
            .unwrap_or_else(|| "-".to_string());

        let span = tracing::info_span!(
            "email_job",
            job_id = %job.id,
            user_id = %user_id,
            trace_id = %trace_id
        );

        if let Some(parent_context) = extract_parent_context(trace_id) {
            span.set_parent(parent_context);
        }

        let _enter = span.enter();

        info!(job_id = %job.id, user_id = %user_id, trace_id = %trace_id, "Processing job");
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
                info!(
                    job_id = %job.id,
                    user_id = %user_id,
                    trace_id = %trace_id,
                    "Job completed successfully."
                );
            }
            Err(e) => {
                error!("Failed to send email: {:?}", e);
                return Err(e);
            }
        }
    }

    Ok(())
}

fn extract_parent_context(traceparent: &str) -> Option<opentelemetry::Context> {
    let mut carrier = std::collections::HashMap::new();
    carrier.insert("traceparent".to_string(), traceparent.to_string());

    let propagator = opentelemetry::global::get_text_map_propagator(|p| p.extract(&carrier));
    Some(propagator)
}
