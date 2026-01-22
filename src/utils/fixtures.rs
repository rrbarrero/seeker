use crate::domain::entities::position::{AppliedOn, Company, Description, InitialComment, Position, RoleTitle, URL};

pub static TESTING_UUID: &str = "67e55044-10b1-426f-9247-bb680e5fe0c8";

pub fn create_fixture_position() -> Position {
    let company = Company::new("hola");
    let role_title = RoleTitle::new("im the role title");
    let description = Description::new("Im the description of the position");
    let applied_on = AppliedOn::new("Tue, 1 Jul 2003 10:52:37 +0200").unwrap();
    let url = URL::new("https://me-the.url");
    let initial_comment = InitialComment::new("... and I the initial comment");

    Position::new_with_uuid(TESTING_UUID, company, role_title, description, applied_on, url, initial_comment).unwrap()
}