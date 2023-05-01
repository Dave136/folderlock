pub mod commands;
pub mod config;
pub mod password;
pub mod store;
pub mod utils;

pub const HASH_START_INDEX: usize = 48;
pub const HASH_STORED_SIZE: usize = 32;
pub const SALT_SIZE: usize = 22;
pub const NONCE_SIZE: usize = 19;
pub const BUFFER_LEN: usize = 500;
