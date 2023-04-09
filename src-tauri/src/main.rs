#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use polars::{prelude::*, datatypes::DataType};
use serde::Serialize;
use tauri::Window;

#[derive(Serialize, Clone, Debug)]
/// Data to be plotted in the frontend
struct Payload {
    shape: (usize, usize),

    // Data as vec of column vecs
    data: Vec<Vec<f32>>,
}

impl From<DataFrame> for Payload {
    fn from(df: DataFrame) -> Self {
        let data = df.iter().map(|s| {
            s.cast(&DataType::Float32).unwrap().f32().unwrap().into_no_null_iter().collect()
        }).collect();

        Payload {
            shape: df.shape(),
            data
        }
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
/// Load CSV and return to the frontent.
fn load_csv(window: Window) {
    let window = window.clone();

    let _ = tauri::api::dialog::FileDialogBuilder::new().pick_file(move |path| {
        // Print data for debug
        // println!("{:?}", path);

        // Load dataframe from a file
        let df = CsvReader::from_path(path.unwrap())
            .unwrap()
            .finish()
            .unwrap();
        // println!("{:?}", a);

        let payload = Payload::from(df);
        // window.emit("event-name", payload).unwrap();
        window.emit("event-name", payload).unwrap();
    });
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, load_csv])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
