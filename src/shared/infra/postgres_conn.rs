use crate::shared::config::Config;
use sqlx::postgres::{PgPool, PgPoolOptions};
use tokio::sync::OnceCell;

static POOL: OnceCell<PgPool> = OnceCell::const_new();

pub async fn get_or_create_pool(config: &Config) -> PgPool {
    POOL.get_or_init(|| async {
        PgPoolOptions::new()
            .connect(&config.postgres_url)
            .await
            .unwrap()
    })
    .await
    .clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_pool() {
        let config = Config::default();
        let pool = get_or_create_pool(&config).await;
        assert!(!pool.is_closed());
    }
}
