use crate::positions::domain::entities::position::{Position, PositionBuilder, PositionStatus};

pub static TESTING_UUID: &str = "67e55044-10b1-426f-9247-bb680e5fe0c8";
pub static TESTING_DATE: &str = "Tue, 1 Jul 2003 10:52:37 +0200";

pub fn create_fixture_position() -> Position {
    let mut builder = PositionBuilder::default();

    builder.with_uuid(TESTING_UUID);
    builder.with_company("hola");
    builder.with_role_title("im the role title");
    builder.with_description("Im the description of the position");
    builder.with_applied_on(TESTING_DATE);
    builder.with_url("https://me-the.url");
    builder.with_initial_comment("... and I the initial comment");
    builder.with_status(PositionStatus::PhoneScreenScheduled);

    builder.build()
}
