use std::env;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Error loading config: `{0}`")]
    InvalidConfiguration(#[from] std::env::VarError),
}

#[derive(PartialEq, Debug)]
pub enum Environment {
    Development,
    Production,
    Test,
}

pub struct Config {
    pub postgres_url: String,
    pub environment: Environment,
}

impl Default for Config {
    fn default() -> Config {
        let environment = match env::var("ENVIRONMENT") {
            Ok(env) => match env.as_str() {
                "development" => Environment::Development,
                "production" => Environment::Production,
                "test" => Environment::Test,
                _ => panic!("Invalid environment"),
            },
            Err(_) => panic!("Environment not set"),
        };

        Config {
            postgres_url: Self::build_database_url(),
            environment,
        }
    }
}

impl Config {
    fn build_database_url() -> String {
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
    fn test_config_load() {
        let config = Config::default();

        assert_eq!(config.environment, Environment::Test);
        let expected_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            std::env::var("POSTGRES_USER").unwrap_or("postgres".to_string()),
            std::env::var("POSTGRES_PASSWORD").unwrap_or("postgres".to_string()),
            std::env::var("POSTGRES_HOST").unwrap_or("localhost".to_string()),
            std::env::var("POSTGRES_PORT").unwrap_or("5432".to_string()),
            std::env::var("POSTGRES_DB").unwrap_or("testdb".to_string())
        );

        assert_eq!(config.postgres_url, expected_url);
    }
}
