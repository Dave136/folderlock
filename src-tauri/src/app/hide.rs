use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, BufWriter};

use tauri::State;

use super::store::Store;

struct Item {
    id: String,
    filename: String,
    content: String,
    password: String,
}

struct HideList {
    id: String,
    items: Option<Vec<Item>>,
    password: Option<String>,
}

#[tauri::command]
pub fn hide_file(path: String, store: State<Store>) {
    let mut file = File::open(path).unwrap();
    // let f = BufReader::new(f);
    let mut output = File::create("example_test.bin").unwrap();
    let mut buffer = Vec::<u8>::new();

    // for line in f.lines() {
    //     println!("{}", line.unwrap());

    store.database.open_tree("app-password").unwrap();
    // }
    file.read_to_end(&mut buffer).unwrap();

    // file.write_all(&buffer.as_slice()).unwrap();
    // file.write_vectored(buffer.to_vec());
    // output.write(&buffer.bytes()).unwrap();
}
