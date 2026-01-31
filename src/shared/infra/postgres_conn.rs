use crate::shared::config::{Config, Environment};
use sqlx::postgres::{PgPool, PgPoolOptions};
use tokio::sync::OnceCell;

static POOL: OnceCell<PgPool> = OnceCell::const_new();

pub async fn get_or_create_pool(config: &Config) -> PgPool {
    if config.environment == Environment::Testing {
        return PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(std::time::Duration::from_secs(30))
            .connect(&config.postgres_url)
            .await
            .expect("Should create pool");
    }

    POOL.get_or_init(|| async {
        PgPoolOptions::new()
            .connect(&config.postgres_url)
            .await
            .expect("Should create pool")
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
