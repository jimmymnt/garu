use std::{error::Error, fs::File, io::BufReader};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::rredis::rredis::RedisManager;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    id: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Users {
    pub users: Vec<User>,
}

#[allow(dead_code)]
impl User {
    pub fn new(
        id: Uuid,
        first_name: String,
        last_name: String,
        email: String,
        ca: String,
        ua: String,
    ) -> Self {
        Self {
            id,
            first_name,
            last_name,
            email,
            created_at: ca,
            updated_at: ua,
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn create(first_name: String, last_name: String, email: String) -> Self {
        let user = Self {
            id: Uuid::new_v4(),
            first_name,
            last_name,
            email,
            created_at: "tmp".to_owned(),
            updated_at: "tmp".to_owned(),
        };

        user
    }
}

impl Users {}
pub fn get_users() -> Result<Users, Box<dyn Error>> {
    // Read users from Json file.
    let path = "../../assets/users.json";
    // Open the file in read-only mode with buffer.
    let file = File::open(path).expect("file should open read only");
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let users = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(users)
}
