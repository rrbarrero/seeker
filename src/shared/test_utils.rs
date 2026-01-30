use crate::auth::domain::entities::user::User;
use crate::shared::config::Config;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use tokio::sync::OnceCell;

use crate::shared::factory::create_postgres_pool;

pub struct TestFactory {
    pub pool: PgPool,
    pub created_users: Vec<Uuid>,
    pub created_positions: Vec<Uuid>,
}

static POOL: OnceCell<PgPool> = OnceCell::const_new();

impl TestFactory {
    pub async fn new() -> Self {
        let pool = POOL
            .get_or_init(|| async {
                let config = Config::default();
                create_postgres_pool(&config).await
            })
            .await
            .clone();

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
        for id in &self.created_users {
            let _ = sqlx::query!("DELETE FROM users WHERE id = $1", id)
                .execute(&self.pool)
                .await;
        }
        for id in &self.created_positions {
            let _ = sqlx::query!("DELETE FROM positions WHERE id = $1", id)
                .execute(&self.pool)
                .await;
        }
    }
}
