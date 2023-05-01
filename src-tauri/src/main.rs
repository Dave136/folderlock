// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;

use app::{
    commands::{has_app_password, hide_file, set_app_password},
    config::Config,
};
use tauri_plugin_store;

use crate::app::store::Store;

const APP_NAME: &'static str = "folderlock";

fn main() {
    let config = Config::new(APP_NAME);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            hide_file,
            set_app_password,
            has_app_password
        ])
        .plugin(tauri_plugin_store::Builder::default().build())
        .manage(Store::new(config.path.store.clone()))
        .manage(config)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
