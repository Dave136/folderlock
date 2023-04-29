use rand::{distributions::Alphanumeric, Rng};
use sled::{Config, Db};

pub struct Store {
    pub database: Db,
    pub path: String,
}

// const DATABASE_PATH: &'static str = "./database";

impl Store {
    pub fn new(path: String) -> Self {
        let sled_config = Config::new()
            .path(&path)
            .temporary(false)
            .use_compression(false);

        let database = sled_config.open().unwrap();

        Self { database, path }
    }

    pub fn rebuild(&self, path: String) -> Self {
        let new_store = Self::new(path);
        new_store
    }

    pub fn generate_id(&self) -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect::<String>()
    }
}
