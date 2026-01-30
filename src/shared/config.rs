use std::env;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Error loading config: `{0}`")]
    InvalidConfiguration(#[from] std::env::VarError),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Environment {
    Production,
    Testing,
}

pub struct Config {
    pub postgres_url: String,
    pub environment: Environment,
}

impl Default for Config {
    fn default() -> Config {
        dotenvy::dotenv().ok();

        let environment = match env::var("ENVIRONMENT")
            .unwrap_or_else(|_| "production".to_string())
            .to_lowercase()
            .as_str()
        {
            "testing" => Environment::Testing,
            _ => Environment::Production,
        };

        let postgres_url = match environment {
            Environment::Testing => "postgres://postgres:postgres@db:5432/testdb".to_string(),
            Environment::Production => Self::build_production_database_url(),
        };

        Config {
            postgres_url,
            environment,
        }
    }
}

impl Config {
    fn build_production_database_url() -> String {
        env::var("DATABASE_URL").unwrap_or_else(|_| {
            let user = env::var("POSTGRES_USER").unwrap_or_else(|_| "postgres".to_string());
            let password = env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "postgres".to_string());
            let host = env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string());
            let port = env::var("POSTGRES_PORT").unwrap_or_else(|_| "5432".to_string());
            let db = env::var("POSTGRES_DB").unwrap_or_else(|_| "develdb".to_string());

            format!("postgres://{user}:{password}@{host}:{port}/{db}")
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_config_load_testing() {
        temp_env::with_var("ENVIRONMENT", Some("testing"), || {
            let config = Config::default();
            assert_eq!(config.environment, Environment::Testing);

            let expected_url = "postgres://postgres:postgres@db:5432/testdb";

            assert_eq!(config.postgres_url, expected_url);
        });
    }

    #[test]
    fn test_config_load_production_default() {
        // Ensure ENVIRONMENT is unset or set to something else
        temp_env::with_var_unset("ENVIRONMENT", || {
            let config = Config::default();
            assert_eq!(config.environment, Environment::Production);
        });
    }
}
