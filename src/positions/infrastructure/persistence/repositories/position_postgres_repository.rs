use crate::shared::config::Config;

pub struct PositionPostgresRepository {
    pool: sqlx::postgres::PgPool,
}

impl PositionPostgresRepository {
    pub async fn new(config: &Config) -> Self {
        Self {
            pool: sqlx::postgres::PgPool::connect(&config.postgres_url)
                .await
                .unwrap(),
        }
    }
}
