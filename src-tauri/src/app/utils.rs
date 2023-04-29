use bcrypt::{hash, verify, DEFAULT_COST};

pub fn hash_password(password: String) -> String {
    hash(password, DEFAULT_COST).unwrap()
}

pub fn verify_password(password: String, hash: &str) -> bool {
    verify(password, hash).unwrap()
}
