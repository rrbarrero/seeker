use std::env;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Error loading config: `{0}`")]
    InvalidConfiguration(#[from] std::env::VarError),
}

pub struct Config {
    pub postgres_url: String,
}

impl Default for Config {
    fn default() -> Config {
        if let Ok(postgres_url) = env::var("DATABASE_URL") {
            Config { postgres_url }
        } else {
            panic!("\nDatabase not configured! Set DATABASE_URL on .env properly.\n")
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_config_load() {
        let config = Config::default();

        assert_ne!(config.postgres_url, "");
    }
}
