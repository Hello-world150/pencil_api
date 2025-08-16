use rocket::serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use serde_json;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

use super::hitokoto::Hitokoto;
use super::user::User;

// This struct holds all the data loaded from files: hitokotos, users, and collections.
#[derive(Serialize, Deserialize)]
pub struct Data {
    pub hitokotos: Vec<Hitokoto>, // All hitokoto records
    pub users: Vec<User>,         // All user records
    pub collections: Vec<String>, // All collection records
}

pub enum DataType {
    Hitokoto,
    User,
    Collection,
}
impl Data {
    // Add hitokoto to data and save to file
    // TODO: Generic type support
    pub fn add(&mut self, hitokoto: Hitokoto) {
        self.hitokotos.push(hitokoto);
        self._save(DataType::Hitokoto);
    }

    fn _save(&mut self, data_type: DataType) {
        match data_type {
            DataType::Hitokoto => {
                // TODO: Implement hitokoto saving
            }
            DataType::User => {
                // TODO: Implement user saving
            }
            DataType::Collection => {
                // TODO: Implement collection saving
            }
        }
    }
}

// Try to open a file. If it doesn't exist, create it and write default JSON content.
// Always returns a file ready for reading or writing.
fn _try_open_file_with_default(path: &str, default_json: &str) -> File {
    // Open the file with read and write access
    match OpenOptions::new().read(true).write(true).open(path) {
        Ok(mut file) => {
            // If file is empty, write default JSON content
            let metadata = file.metadata().expect("Failed to get file metadata");
            if metadata.len() == 0 {
                file.write_all(default_json.as_bytes())
                    .expect(&format!("Failed to write default JSON to {}", path));
                file.flush().expect("Failed to flush file");
            }

            // Rewind the file to the beginning
            file.seek(SeekFrom::Start(0))
                .expect("Failed to rewind file");

            file
        }
        Err(_) => {
            let mut file = File::create(path).expect(&format!("Failed to create {}", path));

            // Write default JSON content
            file.write_all(default_json.as_bytes())
                .expect(&format!("Failed to write default JSON to {}", path));

            // Ensure the file is flushed
            file.flush().expect("Failed to flush file");

            // Reopen the file for reading and writing
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .open(path)
                .expect(&format!("Failed to open {}", path));

            file
        }
    }
}
// Read and parse a JSON file into a Vec<T>.
// If the file does not exist or is empty, it will be created with an empty array.
fn _load_json_vec<T>(path: &str) -> Vec<T>
where
    T: DeserializeOwned,
{
    let mut file = _try_open_file_with_default(path, "[]");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect(&format!("Failed to read {}", path));
    serde_json::from_str(&content).expect(&format!("Failed to parse {}", path))
}

// Load all hitokoto records from hitokoto.json
fn _load_hitokoto_data() -> Vec<Hitokoto> {
    _load_json_vec::<Hitokoto>("hitokoto.json")
}

// Load all user records from user.json
fn _load_user_data() -> Vec<User> {
    _load_json_vec::<User>("user.json")
}

// Load all collection records from collection.json
fn _load_collection_data() -> Vec<String> {
    _load_json_vec::<String>("collection.json")
}
// Load all data (hitokotos, users, collections) and return as a Data struct
pub fn load_data() -> Data {
    let hitokotos = _load_hitokoto_data();
    let users = _load_user_data();
    let collections = _load_collection_data();
    Data {
        hitokotos,
        users,
        collections,
    }
}
