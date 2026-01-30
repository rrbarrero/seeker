use crate::auth::infrastructure::persistence::repositories::user_postgres_repository::UserPostgresRepository;
use crate::positions::infrastructure::persistence::repositories::position_postgres_repository::PositionPostgresRepository;
use crate::shared::config::Config;
use crate::shared::infra::postgres_conn::create_pool;

pub async fn create_postgres_pool(config: &Config) -> sqlx::postgres::PgPool {
    create_pool(config).await
}

pub async fn create_position_postgres_repository(
    pool: sqlx::postgres::PgPool,
) -> PositionPostgresRepository {
    PositionPostgresRepository::new(pool).await
}

pub async fn create_user_postgres_repository(
    pool: sqlx::postgres::PgPool,
) -> UserPostgresRepository {
    UserPostgresRepository::new(pool).await
}
