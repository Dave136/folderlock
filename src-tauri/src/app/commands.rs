use std::path::PathBuf;

use tauri::State;
use uuid::Uuid;

use super::{
    config::Config,
    password::{self, Password}, // utils::{encrypt_file, hash_password},
    store::Store,
};

#[tauri::command]
pub fn set_app_password(password: String, store: State<Store>) {
    let app_tree = store.database.open_tree("app_password").unwrap();
    let encoded = Password::new(password.as_str()).encode().unwrap();

    app_tree.insert("root", encoded).unwrap();
}

#[tauri::command]
pub fn has_app_password(store: State<Store>) -> Result<bool, String> {
    let app_tree = store.database.open_tree("app_password").unwrap();
    let password = app_tree.get("root").unwrap();

    match password {
        Some(pass) => {
            let decoded: String = bincode::deserialize(&pass).unwrap();

            if decoded.is_empty() {
                Ok(false)
            } else {
                Ok(true)
            }
        }
        None => Err(String::from("No password found")),
    }
}

#[tauri::command]
pub fn hide_file(filepath: String, store: State<Store>, config: State<Config>) {
    let id: Uuid = Uuid::new_v4();
    let location = PathBuf::from(config.path.backup.clone()).join(id.to_string());

    let key = b"th1s_1s_my_s3cr3t_k3y_m0r3_byt3s";
    let nonce = b"nonce_12_bytes";

    println!("my custom location: {:?}", location);

    // encrypt_file(key, filepath.as_str(), "", nonce).unwrap();

    // let file = File::open(filepath).unwrap();
    // let mut f = BufReader::new(file);
    // let mut output = File::create("example_test.txt").unwrap();
    // let mut buffer = Vec::<u8>::new();

    // f.read_to_end(&mut buffer).unwrap();

    // output.write(&buffer).unwrap();
}
