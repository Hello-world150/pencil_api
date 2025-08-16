use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct ErrorMessage(pub &'static str);
