use async_trait::async_trait;

use crate::{
    auth::{
        domain::{entities::user::User, repositories::user_repository::IUserRepository},
        infrastructure::persistence::repositories::dtos::UserDto,
    },
    shared::domain::{error::AuthRepositoryError, value_objects::UserUuid},
};

pub struct UserPostgresRepository {
    pool: sqlx::postgres::PgPool,
}

impl UserPostgresRepository {
    pub async fn new(pool: sqlx::postgres::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl IUserRepository for UserPostgresRepository {
    async fn save(&self, user: &User) -> Result<UserUuid, AuthRepositoryError> {
        sqlx::query!(
            "INSERT INTO users (id, email, password, created_at, updated_at) VALUES ($1, $2, $3, $4, $5)",
            user.id.value(),
            user.email.value(),
            user.password().value(),
            user.created
                .and_hms_opt(0, 0, 0)
                .expect("Created date should be valid")
                .and_utc(),
            user.updated
                .and_hms_opt(0, 0, 0)
                .expect("Updated date should be valid")
                .and_utc(),
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AuthRepositoryError::DatabaseError(e.to_string()))?;

        Ok(user.id)
    }
    async fn get(&self, user_id: UserUuid) -> Result<Option<User>, AuthRepositoryError> {
        let result = sqlx::query("SELECT * FROM users WHERE id = $1")
            .bind(user_id.value())
            .fetch_optional(&self.pool)
            .await;

        match result {
            Ok(Some(row)) => UserDto::from_row(&row)
                .to_domain()
                .map(Some)
                .map_err(AuthRepositoryError::from),
            Ok(None) => Ok(None),
            Err(e) => Err(AuthRepositoryError::DatabaseError(e.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::infrastructure::test_factory::TestFactory;

    #[tokio::test]
    async fn test_save_user_postgres_repository() {
        let factory = TestFactory::new().await;

        let pool = factory.pool.clone();
        let repository = UserPostgresRepository::new(pool).await;

        let id = uuid::Uuid::new_v4();
        let email = format!("test.{}@example.com", id);
        let password = "S0m3V3ryStr0ngP@ssw0rd!";
        let user = User::new(&id.to_string(), &email, password).expect("User creation failed");

        let result = repository.save(&user).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_user_postgres_repository() {
        let expected_password = "S0m3V3ryStr0ngP@ssw0rd!";
        let mut factory = TestFactory::new().await;

        let created_user = factory.create_random_user().await;

        let pool = factory.pool.clone();
        let repository = UserPostgresRepository::new(pool).await;

        let result = repository.get(created_user.id).await;

        assert!(result.is_ok());
        let user = result.expect("Error getting user").expect("User not found");
        assert_eq!(user.id, created_user.id);
        assert_eq!(user.email.value(), created_user.email.value());

        assert!(
            user.verify_password(expected_password)
                .expect("Verification failed")
        );
    }

    #[tokio::test]
    async fn test_repository_contract() {
        let factory = TestFactory::new().await;
        let pool = factory.pool.clone();
        let repository = UserPostgresRepository::new(pool).await;

        let id = uuid::Uuid::new_v4();
        let email = format!("test.{}@example.com", id);
        let password = "S0m3V3ryStr0ngP@ssw0rd!";
        let user = User::new(&id.to_string(), &email, password).expect("User creation failed");

        crate::auth::infrastructure::persistence::repositories::common_repository_tests::assert_user_repository_behavior(
            Box::new(repository),
            user,
        )
        .await;
    }
}
