use crate::auth::domain::entities::user::User;
use crate::shared::config::Config;
use crate::shared::factory::get_or_create_postgres_pool;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

pub struct TestFactory {
    pub pool: PgPool,
    pub created_users: Vec<Uuid>,
    pub created_positions: Vec<Uuid>,
}

impl TestFactory {
    pub async fn new() -> Self {
        let config = Config::default();
        let pool = get_or_create_postgres_pool(&config).await;

        Self {
            pool,
            created_users: Vec::new(),
            created_positions: Vec::new(),
        }
    }

    pub async fn create_random_user(&mut self) -> User {
        let id = Uuid::new_v4();
        let email = format!("test.{}@example.com", id);
        let password = "S0m3V3ryStr0ngP@ssw0rd!";

        sqlx::query!(
            "INSERT INTO users (id, email, password, created_at, updated_at) VALUES ($1, $2, $3, $4, $5)",
            id,
            email,
            password,
            Utc::now(),
            Utc::now()
        )
        .execute(&self.pool)
        .await
        .expect("Failed to create random user");

        self.created_users.push(id);

        User::new(&id.to_string(), &email, password).expect("Invalid user created in factory")
    }

    pub async fn teardown(&self) {
        // Delete all positions for our created users first to avoid FK violations
        for user_id in &self.created_users {
            let _ = sqlx::query!("DELETE FROM positions WHERE user_id = $1", user_id)
                .execute(&self.pool)
                .await;
        }

        // Now delete individual positions that might not have been caught (though shouldn't exist)
        for id in &self.created_positions {
            let _ = sqlx::query!("DELETE FROM positions WHERE id = $1", id)
                .execute(&self.pool)
                .await;
        }

        // Finally delete users
        for id in &self.created_users {
            let _ = sqlx::query!("DELETE FROM users WHERE id = $1", id)
                .execute(&self.pool)
                .await;
        }
    }
}
