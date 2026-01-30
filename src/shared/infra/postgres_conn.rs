use crate::shared::config::Config;

pub async fn create_pool(config: &Config) -> sqlx::postgres::PgPool {
    sqlx::postgres::PgPool::connect(&config.postgres_url)
        .await
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_pool() {
        let config = Config::default();
        let pool = create_pool(&config).await;
        assert!(!pool.is_closed());
    }
}
