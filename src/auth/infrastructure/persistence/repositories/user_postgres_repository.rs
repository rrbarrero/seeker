use async_trait::async_trait;

use crate::{
    auth::domain::{entities::user::User, repositories::user_repository::IUserRepository},
    shared::{
        config::Config,
        domain::{error::UserValueError, value_objects::UserUuid},
    },
};

pub struct UserPostgresRepository {
    pool: sqlx::postgres::PgPool,
}

impl UserPostgresRepository {
    pub async fn new(config: &Config) -> Self {
        Self {
            pool: sqlx::postgres::PgPool::connect(&config.postgres_url)
                .await
                .unwrap(),
        }
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

    use std::str::FromStr;

    use super::*;
    use crate::shared::{
        config::Config,
        db_sync,
        fixtures::{TESTING_EMAIL, TESTING_PASSWORD, TESTING_UUID_1, TESTING_UUID_2},
    };

    async fn init_test_database() {
        let config = Config::default();
        let _ = db_sync::db_sync(&config).await;
    }

    async fn delete_user(config: &Config, user_id: &str) {
        let pool = sqlx::postgres::PgPool::connect(&config.postgres_url)
            .await
            .unwrap();
        sqlx::query("DELETE FROM users WHERE id = $1::uuid")
            .bind(user_id)
            .execute(&pool)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_save_user_postgres_repository() {
        init_test_database().await;

        let config = Config::default();

        delete_user(&config, TESTING_UUID_1).await;

        let mut repository = UserPostgresRepository::new(&config).await;

        let user = User::new(TESTING_UUID_1, TESTING_EMAIL, TESTING_PASSWORD)
            .expect("Failed to create user!");

        let result = repository.save(&user).await;

        if let Err(e) = &result {
            println!("Error saving user: {:?}", e);
        }
        assert!(result.is_ok());

        delete_user(&config, TESTING_UUID_1).await;
    }

    #[tokio::test]
    async fn test_get_user_postgres_repository() {
        init_test_database().await;

        let config = Config::default();
        delete_user(&config, TESTING_UUID_2).await;

        let mut repository = UserPostgresRepository::new(&config).await;

        let user = User::new(TESTING_UUID_2, TESTING_EMAIL, TESTING_PASSWORD)
            .expect("Failed to create user!");

        let result = repository.save(&user).await;

        if let Err(e) = &result {
            println!("Error saving user: {:?}", e);
        }
        assert!(result.is_ok());

        let result = repository
            .get(UserUuid::from_str(TESTING_UUID_2).expect("Failed to create user!"))
            .await;

        if let None = &result {
            println!("Error getting user: {:?}", result);
        }
        assert!(result.is_some());

        delete_user(&config, TESTING_UUID_2).await;
    }
}
