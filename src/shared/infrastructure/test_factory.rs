use crate::auth::domain::entities::user::User;
use crate::auth::domain::repositories::user_repository::IUserRepository;
use crate::auth::infrastructure::persistence::repositories::user_postgres_repository::UserPostgresRepository;
use crate::composition_root::get_or_create_postgres_pool;
use crate::shared::config::Config;
use sqlx::PgPool;
use uuid::Uuid;

use std::sync::{Arc, Mutex};

pub struct TestFactory {
    pub pool: PgPool,
    state: Arc<Mutex<FactoryState>>,
}

struct FactoryState {
    created_users: Vec<Uuid>,
    created_positions: Vec<Uuid>,
}

impl TestFactory {
    pub async fn new() -> Self {
        let config = Config::default();
        let pool = get_or_create_postgres_pool(&config).await;

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to migrate database");

        Self {
            pool,
            state: Arc::new(Mutex::new(FactoryState {
                created_users: Vec::new(),
                created_positions: Vec::new(),
            })),
        }
    }

    pub async fn create_random_user(&mut self) -> User {
        let id = Uuid::new_v4();
        let email = format!("test.{}@example.com", id);
        let password = "S0m3V3ryStr0ngP@ssw0rd!";
        let user = User::new(&id.to_string(), &email, password).expect("User creation failed");

        let mut repository = UserPostgresRepository::new(self.pool.clone()).await;
        repository
            .save(&user)
            .await
            .expect("Failed to save user through repository in factory");

        self.state.lock().unwrap().created_users.push(id);

        user
    }

    pub async fn teardown(&self) {
        let (users, positions) = {
            let state = self.state.lock().unwrap();
            (state.created_users.clone(), state.created_positions.clone())
        };

        // Delete all positions for our created users first to avoid FK violations
        for user_id in &users {
            let _ = sqlx::query!("DELETE FROM positions WHERE user_id = $1", user_id)
                .execute(&self.pool)
                .await;
        }

        // Now delete individual positions
        for id in &positions {
            let _ = sqlx::query!("DELETE FROM positions WHERE id = $1", id)
                .execute(&self.pool)
                .await;
        }

        // Finally delete users
        for id in &users {
            let _ = sqlx::query!("DELETE FROM users WHERE id = $1", id)
                .execute(&self.pool)
                .await;
        }

        let mut state = self.state.lock().unwrap();
        state.created_users.clear();
        state.created_positions.clear();
    }
    pub fn track_position(&mut self, id: Uuid) {
        self.state.lock().unwrap().created_positions.push(id);
    }
}

impl Drop for TestFactory {
    fn drop(&mut self) {
        let state = self.state.lock().unwrap();
        if !state.created_users.is_empty() || !state.created_positions.is_empty() {
            let pool = self.pool.clone();
            let users = state.created_users.clone();
            let positions = state.created_positions.clone();

            // Drop can't be async, so we spawn a thread to handle the cleanup
            // We use a new runtime to block on the async cleanup
            std::thread::spawn(move || {
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap();

                rt.block_on(async move {
                    for user_id in &users {
                        let _ = sqlx::query!("DELETE FROM positions WHERE user_id = $1", user_id)
                            .execute(&pool)
                            .await;
                    }
                    for id in &positions {
                        let _ = sqlx::query!("DELETE FROM positions WHERE id = $1", id)
                            .execute(&pool)
                            .await;
                    }
                    for id in &users {
                        let _ = sqlx::query!("DELETE FROM users WHERE id = $1", id)
                            .execute(&pool)
                            .await;
                    }
                });
            });
        }
    }
}
