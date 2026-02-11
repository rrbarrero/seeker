use async_trait::async_trait;

use crate::auth::application::errors::AuthError;

#[async_trait]
pub trait IEmailQueueEnqueuer: Send + Sync {
    async fn enqueue(
        &self,
        to: &str,
        subject: &str,
        body: &str,
        user_id: uuid::Uuid,
        trace_id: Option<String>,
    ) -> Result<(), AuthError>;
}
