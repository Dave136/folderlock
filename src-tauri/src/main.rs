// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;

use std::path::PathBuf;

use app::{
    commands::{has_app_password, set_app_password},
    hide::hide_file,
};
use tauri_plugin_store;

use crate::app::store::Store;

fn main() {
    let path = PathBuf::from("/home/dave/.cache/folderlock/database")
        .into_os_string()
        .into_string()
        .unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            hide_file,
            set_app_password,
            has_app_password
        ])
        .plugin(tauri_plugin_store::Builder::default().build())
        .manage(Store::new(path))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
