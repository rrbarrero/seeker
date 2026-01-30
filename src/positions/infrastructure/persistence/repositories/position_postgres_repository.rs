use sqlx::postgres::PgPool;

pub struct PositionPostgresRepository {
    pool: PgPool,
}

impl PositionPostgresRepository {
    pub async fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
