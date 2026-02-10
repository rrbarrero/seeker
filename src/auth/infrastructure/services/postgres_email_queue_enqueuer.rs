use async_trait::async_trait;

use crate::auth::application::{email_queue_enqueuer::IEmailQueueEnqueuer, errors::AuthError};

pub struct PostgresEmailQueueEnqueuer {
    pool: sqlx::postgres::PgPool,
}

impl PostgresEmailQueueEnqueuer {
    pub fn new(pool: sqlx::postgres::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl IEmailQueueEnqueuer for PostgresEmailQueueEnqueuer {
    async fn enqueue(&self, to: &str, subject: &str, body: &str) -> Result<(), AuthError> {
        let payload = serde_json::json!({
            "to": to,
            "subject": subject,
            "body": body
        });

        sqlx::query!("INSERT INTO email_queue (payload) VALUES ($1)", payload)
            .execute(&self.pool)
            .await
            .map_err(|e| AuthError::InternalError(e.to_string()))?;

        Ok(())
    }
}
