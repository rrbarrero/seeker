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
#[derive(Debug, Clone)]
pub struct Config {
    pub postgres_url: String,
    pub environment: Environment,
    pub server_host: String,
    pub server_port: u16,
    jwt_secret: String,
    pub jwt_expiration_time: i64,
}

impl Default for Config {
    fn default() -> Config {
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

        if environment != Environment::Testing && env::var("JWT_SECRET").is_err() {
            panic!("JWT_SECRET is not set");
        }

        Config {
            postgres_url,
            environment,
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),
            jwt_secret: env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string()),
            jwt_expiration_time: env::var("JWT_EXPIRATION_TIME")
                .unwrap_or_else(|_| "10800".to_string())
                .parse()
                .unwrap_or(60 * 60 * 3),
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

    pub fn get_jwt_secret(&self) -> String {
        self.jwt_secret.clone()
    }

    #[cfg(test)]
    pub fn test_default() -> Self {
        Self {
            postgres_url: "postgres://postgres:postgres@db:5432/testdb".to_string(),
            environment: Environment::Testing,
            server_host: "127.0.0.1".to_string(),
            server_port: 3000,
            jwt_secret: "secret".to_string(),
            jwt_expiration_time: 60 * 60 * 3,
        }
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
        temp_env::with_var("JWT_SECRET", Some("secret"), || {
            temp_env::with_var_unset("ENVIRONMENT", || {
                let config = Config::default();
                assert_eq!(config.environment, Environment::Production);
            });
        });
    }

    #[test]
    #[should_panic(expected = "JWT_SECRET is not set")]
    fn test_should_panic_when_jwt_secret_is_not_set_and_env_is_production() {
        temp_env::with_var_unset("JWT_SECRET", || {
            temp_env::with_var_unset("ENVIRONMENT", || {
                let _ = Config::default();
            });
        })
    }
}
