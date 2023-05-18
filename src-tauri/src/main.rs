// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod savedata;
mod stocks;
mod types;

fn main() {
    savedata::check_save();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            stocks::buy_stock,
            stocks::sell_stock,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
