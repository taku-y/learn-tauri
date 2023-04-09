#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use polars::prelude::*;
use serde::Serialize;
use tauri::Window;

#[derive(Serialize, Clone)]
struct Payload {
    data: String,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn load_csv(window: Window) {
    let window = window.clone();
    println!("{:?}", window);
    let _ = tauri::api::dialog::FileDialogBuilder::new().pick_file(move |path| {
        println!("{:?}", path);
        let a = CsvReader::from_path(path.unwrap())
            .unwrap()
            .finish()
            .unwrap();
        println!("{:?}", a);
        window.emit("event-name", Payload { data: a.to_string() });
    });
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, load_csv])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
