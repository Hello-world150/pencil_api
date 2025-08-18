use rocket::serde::{Deserialize, Serialize};

/// The struct defination of the hitokotos.
#[derive(Deserialize, Serialize, Clone)]
pub struct Anthology {
    id: u64,
    hitokotos: Vec<u64>,
    user_id: u64,
}
