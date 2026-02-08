use crate::auth::application::auth_service::AuthService;
use crate::auth::domain::repositories::user_repository::IUserRepository;
use crate::auth::infrastructure::persistence::repositories::user_in_memory_repository::UserInMemoryRepository;
use crate::auth::infrastructure::persistence::repositories::user_postgres_repository::UserPostgresRepository;
use crate::auth::infrastructure::services::jwt_token_generator::JwtTokenGenerator;
use crate::positions::application::position_service::PositionService;
use crate::positions::domain::repositories::position_repository::IPositionRepository;
use crate::positions::infrastructure::persistence::repositories::position_postgres_repository::PositionPostgresRepository;
use crate::shared::config::Config;
use crate::shared::infrastructure::postgres_conn::get_or_create_pool;
use std::sync::Arc;

pub async fn get_or_create_postgres_pool(config: &Config) -> sqlx::postgres::PgPool {
    get_or_create_pool(config).await
}

pub async fn create_position_postgres_repository(
    pool: sqlx::postgres::PgPool,
) -> PositionPostgresRepository {
    PositionPostgresRepository::new(pool).await
}

pub async fn create_user_in_memory_repository() -> UserInMemoryRepository {
    UserInMemoryRepository::default()
}

pub async fn create_user_postgres_repository(
    pool: sqlx::postgres::PgPool,
) -> UserPostgresRepository {
    UserPostgresRepository::new(pool).await
}

pub async fn create_position_service(repo: Box<dyn IPositionRepository>) -> PositionService {
    PositionService::new(repo)
}

pub async fn create_auth_service(
    repo: Box<dyn IUserRepository>,
    config: Arc<Config>,
) -> AuthService {
    let token_generator = Box::new(JwtTokenGenerator::new(config));
    AuthService::new(repo, token_generator)
}
