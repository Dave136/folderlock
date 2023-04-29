use tauri::State;

use super::{store::Store, utils::hash_password};

#[tauri::command]
pub fn set_app_password(password: String, store: State<Store>) {
    let app_tree = store.database.open_tree("app_password").unwrap();
    let encrypted = hash_password(password);
    let encoded = bincode::serialize(&encrypted).unwrap();
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
