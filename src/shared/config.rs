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
        let postgres_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Config {
            postgres_url,
            environment,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_config_load() {
        let config = Config::default();

        assert_eq!(config.environment, Environment::Test);
        assert_eq!(
            config.postgres_url,
            "postgres://postgres:postgres@db:5432/postgres"
        );
    }
}
