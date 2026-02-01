use sqlx::{Row, postgres::PgRow};

use crate::{auth::domain::entities::user::User, shared::domain::error::UserValueError};

#[derive(Clone, Debug)]
pub struct UserDto {
    id: uuid::Uuid,
    email: String,
    password: String,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}

impl UserDto {
    pub fn from_row(row: &PgRow) -> Self {
        Self {
            id: row.get("id"),
            email: row.get("email"),
            password: row.get("password"),
            created_at: row
                .get::<chrono::DateTime<chrono::Utc>, _>("created_at")
                .naive_utc(),
            updated_at: row
                .get::<chrono::DateTime<chrono::Utc>, _>("updated_at")
                .naive_utc(),
        }
    }

    pub fn from_domain(user: &User) -> Result<Self, UserValueError> {
        Ok(Self {
            id: user.id.value(),
            email: user.email.value().to_string(),
            password: user.password().value().to_string(),
            created_at: user
                .created
                .and_hms_opt(0, 0, 0)
                .ok_or(UserValueError::InvalidDateTime)?
                .and_utc()
                .naive_utc(),

            updated_at: user
                .updated
                .and_hms_opt(0, 0, 0)
                .ok_or(UserValueError::InvalidDateTime)?
                .and_utc()
                .naive_utc(),
        })
    }

    pub fn to_domain(self) -> Result<User, UserValueError> {
        User::load_existing(
            &self.id.to_string(),
            &self.email,
            &self.password,
            self.created_at.date(),
            self.updated_at.date(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::infrastructure::test_factory::TestFactory;

    #[tokio::test]
    async fn test_user_dto_from_domain() {
        let mut factory = TestFactory::new().await;
        let user = factory.create_random_user().await;

        let dto = UserDto::from_domain(&user).expect("Error creating user dto");

        assert_eq!(dto.id, user.id.value());
        assert_eq!(dto.email, user.email.value());
        assert_eq!(dto.password, user.password().value());
        assert_eq!(
            dto.created_at,
            user.created
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_utc()
                .naive_utc()
        );
        assert_eq!(
            dto.updated_at,
            user.updated
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_utc()
                .naive_utc()
        );
    }

    #[tokio::test]
    async fn test_user_dto_to_domain() {
        let mut factory = TestFactory::new().await;
        let user = factory.create_random_user().await;

        let dto = UserDto::from_domain(&user).expect("Error creating user dto");
        let result = dto.to_domain();

        assert!(result.is_ok());
        assert_eq!(result.unwrap().id, user.id);
    }

    #[tokio::test]
    async fn test_user_dto_from_row() {
        let mut factory = TestFactory::new().await;
        let user = factory.create_random_user().await;

        let query_result = sqlx::query("SELECT * FROM users WHERE id = $1")
            .bind(user.id.value())
            .fetch_optional(&factory.pool)
            .await;

        assert!(query_result.is_ok());
        assert!(query_result.as_ref().unwrap().is_some());
        let row = query_result
            .expect("Error getting row")
            .expect("Row not found");

        let dto = UserDto::from_row(&row);
        let result = dto.clone().to_domain();

        assert!(result.is_ok());
        assert_eq!(result.unwrap().id, user.id);

        assert_eq!(dto.id, user.id.value());
        assert_eq!(dto.email, user.email.value());
        assert_eq!(dto.password, user.password().value());
        assert_eq!(dto.created_at.date(), user.created);
        assert_eq!(dto.updated_at.date(), user.updated);
    }
}
