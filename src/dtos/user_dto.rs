use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct UserDTO {
    pub username: Box<str>,
    pub biography: Box<str>,
    pub birthdate: chrono::NaiveDate,
}
