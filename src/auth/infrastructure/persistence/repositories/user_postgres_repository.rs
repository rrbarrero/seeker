use async_trait::async_trait;

use crate::{
    auth::domain::{entities::user::User, repositories::user_repository::IUserRepository},
    shared::domain::{error::UserValueError, value_objects::UserUuid},
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
    async fn save(&mut self, user: &User) -> Result<UserUuid, UserValueError> {
        sqlx::query!(
            "INSERT INTO users (id, email, password, created_at, updated_at) VALUES ($1, $2, $3, $4, $5)",
            user.id.value(),
            user.email.value(),
            user.password().value(),
            user.created.and_hms_opt(0, 0, 0).unwrap().and_utc(),
            user.updated.and_hms_opt(0, 0, 0).unwrap().and_utc(),
        )
        .execute(&self.pool)
        .await?;

        Ok(user.id)
    }
    async fn get(&self, user_id: UserUuid) -> Option<User> {
        let user: Option<Result<User, UserValueError>> =
            sqlx::query!("SELECT * FROM users WHERE id = $1", user_id.value())
                .fetch_one(&self.pool)
                .await
                .ok()
                .map(|user| User::new(&user.id.to_string(), &user.email, &user.password));
        user.unwrap().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::test_utils::TestFactory;

    #[tokio::test]
    async fn test_save_user_postgres_repository() {
        let factory = TestFactory::new().await;

        let pool = factory.pool.clone();
        let mut repository = UserPostgresRepository::new(pool).await;

        let id = uuid::Uuid::new_v4();
        let email = format!("test.{}@example.com", id);
        let password = "S0m3V3ryStr0ngP@ssw0rd!";
        let user = User::new(&id.to_string(), &email, password).expect("User creation failed");

        let result = repository.save(&user).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_user_postgres_repository() {
        let mut factory = TestFactory::new().await;

        let user = factory.create_random_user().await;

        let pool = factory.pool.clone();
        let repository = UserPostgresRepository::new(pool).await;

        let result = repository.get(user.id.clone()).await;

        assert!(result.is_some());
        assert_eq!(result.unwrap().id, user.id);

        factory.teardown().await;
    }
}
