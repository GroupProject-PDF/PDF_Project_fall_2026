#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod pdf;
mod db;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::merge_pdfs_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
