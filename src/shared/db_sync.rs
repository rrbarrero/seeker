use sqlx::postgres::PgPoolOptions;

use crate::shared::config::Config;

pub async fn db_sync(config: &Config) -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.postgres_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(())
}
