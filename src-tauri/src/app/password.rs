use std::error::Error;

use argon_hash_password::{check_password_matches_hash, create_hash_and_salt};
use serde::{Deserialize, Serialize};

pub struct Password {
    val: String,
    pub hash: String,
    pub salt: String,
}

#[derive(Serialize, Deserialize)]
pub struct PasswordDto {
    pub hash: String,
    pub salt: String,
}

impl Password {
    pub fn new(val: &str) -> Self {
        let (hash, salt) = Self::generate_hash(val).unwrap();
        Self {
            val: val.to_owned(),
            hash: hash.to_owned(),
            salt: salt.to_owned(),
        }
    }

    pub fn encode(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let dto = PasswordDto {
            hash: self.hash.clone(),
            salt: self.salt.clone(),
        };

        let encoded = bincode::serialize(&dto)?;

        Ok(encoded)
    }

    fn generate_hash(password: &str) -> Result<(String, String), Box<dyn Error>> {
        create_hash_and_salt(password)
    }

    pub fn compare(password: &str, salt: &str, hash: &str) -> Result<bool, Box<dyn Error>> {
        check_password_matches_hash(password, hash, salt)
    }
}
