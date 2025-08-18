use rocket::serde::{Deserialize, Serialize};

/// The struct defination of the hitokotos.
#[derive(Deserialize, Serialize, Clone)]
pub struct Anthology {
    id: u64,
    hitokotos: Vec<u64>,
    user_id: u64,
}

impl Anthology {
    pub fn from(new_anthology: NewAnthology) -> Self {
        // Generate new id with snowflaked
        let id = snowflaked::Generator::new(0).generate();

        Anthology {
            id,
            hitokotos: new_anthology.hitokotos,
            user_id: new_anthology.user_id,
        }
    }
}
/// This struct b used to generate new anthology.
/// This struct has no `id`.
#[derive(Deserialize)]
pub struct NewAnthology {
    pub hitokotos: Vec<u64>,
    pub user_id: u64,
}
