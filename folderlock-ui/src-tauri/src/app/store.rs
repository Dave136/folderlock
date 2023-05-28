use std::path::PathBuf;

use sled::{Config, Db};

pub struct Store {
    pub database: Db,
}

impl Store {
    pub fn new(path: PathBuf) -> Self {
        let sled_config = Config::new()
            .path(&path)
            .temporary(false)
            .use_compression(false);

        let database = sled_config.open().unwrap();

        Self { database }
    }
}
