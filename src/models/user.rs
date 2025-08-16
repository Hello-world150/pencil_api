use bcrypt::BcryptResult;
use rocket::serde::{Deserialize, Serialize};
/// The struct defination of the users.
#[derive(Serialize, Deserialize)]
pub struct User {
    /// The user id of the user.
    id: u64,
    /// The email of the user.
    email: String,
    /// The hashed password of the user.
    password: String,
    /// The hitokoto id vector of the user,
    /// be used to query the hitokotos that this user owned.
    hitokotos: Vec<u64>,
}

impl User {
    /// Generate new user from `NewUser` struct.
    pub fn from(new_user: NewUser) -> Self {
        // Generate new id with snowflaked
        let id = snowflaked::Generator::new(0).generate();

        // Generate bcrypt hashed password with cost 10
        let hashed_password = bcrypt::hash(new_user.raw_password, 10).unwrap();

        // Generate new vector for hitokoto ids
        let hitokotos = Vec::new();
        User {
            id,
            email: new_user.email,
            password: hashed_password,
            hitokotos,
        }
    }

    /// Verify password.
    pub fn verify(&self, password: String) -> BcryptResult<bool> {
        bcrypt::verify(password, &self.password)
    }
}

/// This struct be used to generate new user.
/// This struct has no `id`, `hashed password` and `hitokoto vector`.
#[derive(Deserialize)]
pub struct NewUser {
    email: String,
    raw_password: String,
}
