use crate::positions::domain::entities::position::{Position, PositionBuilder, PositionStatus};

pub static TESTING_UUID: &str = "67e55044-10b1-426f-9247-bb680e5fe0c8";
pub static TESTING_DATE: &str = "Fri, 23 Jan 2026 10:10:10 +0200";
pub static TESTING_EMAIL: &str = "test@example.com";
pub static TESTING_PASSWORD: &str = "klerjLKj2k3jfvkhf,uiuhNJLK2)(/";

pub fn create_fixture_position() -> Position {
    PositionBuilder::default()
        .with_uuid(TESTING_UUID)
        .unwrap()
        .with_company("hola")
        .with_role_title("im the role title")
        .with_description("Im the description of the position")
        .with_applied_on(TESTING_DATE)
        .unwrap()
        .with_url("https://me-the.url")
        .with_initial_comment("... and I the initial comment")
        .with_status(PositionStatus::PhoneScreenScheduled)
        .build()
}
