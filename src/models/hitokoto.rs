use super::data::Data;
use rocket::serde::{Deserialize, Serialize};

/// The struct defination of the hitokotos.
#[derive(Serialize, Deserialize, Clone)]
pub struct Hitokoto {
    /// The id of the hitokoto.
    id: i64,
    /// The content of the hitokoto.
    content: String,
    /// The source of the hitokoto, author or book etc.
    from: String,
    /// The optional character of the hitokoto.
    from_who: Option<String>,
    /// The user id of the hitokoto, be used to query the owner.
    user_id: i64,
    /// The create time of the hitokoto.
    created_at: u64,
}

impl Hitokoto {
    pub fn from(new_hitokoto: NewHitokoto) -> Self {
        // Generate new id with snowflaked
        let id = snowflaked::Generator::new(0).generate();

        // Submit time, in seconds
        let created_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        Hitokoto {
            id: id,
            content: new_hitokoto.content,
            from: new_hitokoto.from,
            from_who: new_hitokoto.from_who,
            user_id: new_hitokoto.user_id,
            created_at: created_at,
        }
    }
}

/// This struct be used to generate new hitokoto.
/// This struct has no `id`.
#[derive(Deserialize)]
pub struct NewHitokoto {
    pub content: String,
    pub from: String,
    pub from_who: Option<String>,
    pub user_id: i64,
}

/// Get a random hitokoto from the data.
pub fn get_random_hitokoto(data: &Data, rng: &mut impl rand::Rng) -> Option<Hitokoto> {
    if data.hitokotos.is_empty() {
        return None;
    }
    // Generate a random index
    let random_index = rng.random_range(0..data.hitokotos.len());
    Some(data.hitokotos[random_index].clone())
}
