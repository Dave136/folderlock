use std::path::PathBuf;

pub struct ConfigPath {
    pub store: PathBuf,
    pub backup: PathBuf,
    pub root: PathBuf,
}

pub struct Config {
    pub path: ConfigPath,
}

impl Config {
    pub fn new(root: &str) -> Self {
        let cache_dir = Self::cache_dir(root).unwrap();

        let path = ConfigPath {
            root: cache_dir.clone(),
            backup: cache_dir.join("backup"),
            store: cache_dir.join("database"),
        };

        Self { path }
    }

    fn cache_dir(app_name: &str) -> Option<PathBuf> {
        dirs_next::cache_dir().map(|dir| dir.join(app_name))
    }
}
