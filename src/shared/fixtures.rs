use uuid::Uuid;

use crate::positions::domain::entities::position::{Position, PositionBuilder, PositionStatus};

pub static TESTING_DATE: &str = "Fri, 23 Jan 2026 10:10:10 +0200";

pub fn create_fixture_position() -> Position {
    PositionBuilder::default()
        .with_uuid(&Uuid::new_v4().to_string())
        .expect("Should create position with uuid")
        .with_user_uuid(&Uuid::new_v4().to_string())
        .expect("Should create position with user uuid")
        .with_company("hola")
        .with_role_title("im the role title")
        .with_description("Im the description of the position")
        .with_applied_on(TESTING_DATE)
        .expect("Should create position with applied on")
        .with_url("https://me-the.url")
        .with_initial_comment("... and I the initial comment")
        .with_status(PositionStatus::PhoneScreenScheduled)
        .build()
}

pub fn valid_email() -> &'static str {
    "test@example.com"
}
pub fn valid_password() -> &'static str {
    "S0m3V3ryStr0ngP@ssw0rd!"
}
pub fn valid_id() -> String {
    Uuid::new_v4().to_string()
}
