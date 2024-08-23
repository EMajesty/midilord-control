// src/app/mod.rs
// Contains Tauri runtime code

mod handlers;
mod commands;
mod generate_test_data;
mod structs;
mod constants;

/// Run Tauri builder to initialize the rust runtime.
pub fn initialize_tauri(){
  tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![commands::update_device_config, commands::connect_device])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}