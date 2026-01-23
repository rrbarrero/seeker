use crate::{auth::domain::entities::user::User, shared::config::Config};

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

    pub async fn save(&self, user: &User) -> Result<(), sqlx::Error> {
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

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::shared::{
        config::Config,
        db_sync,
        fixtures::{TESTING_EMAIL, TESTING_PASSWORD, TESTING_UUID},
    };

    async fn init_test_database() {
        let config = Config::default();
        let _ = db_sync::db_sync(&config).await;
    }

    async fn clear_users_table(config: &Config) {
        let pool = sqlx::postgres::PgPool::connect(&config.postgres_url)
            .await
            .unwrap();
        let _ = sqlx::query!("TRUNCATE TABLE users")
            .execute(&pool)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_save_user_postgres_repository() {
        init_test_database().await;

        let config = Config::default();
        let repository = UserPostgresRepository::new(&config).await;

        let user = User::new(TESTING_UUID, TESTING_EMAIL, TESTING_PASSWORD)
            .expect("Failed to create user!");

        let result = repository.save(&user).await;

        if let Err(e) = &result {
            println!("Error saving user: {:?}", e);
        }
        assert!(result.is_ok());

        clear_users_table(&config).await;
    }
}
