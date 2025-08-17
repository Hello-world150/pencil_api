use rocket::serde::{Deserialize, Serialize};

/// The struct defination of the hitokotos.
#[derive(Deserialize, Serialize)]
pub struct Collection {
    id: u64,
    hitokotos: Vec<u64>,
    user_id: u64,
}
