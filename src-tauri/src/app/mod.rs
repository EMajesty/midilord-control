// src/app/mod.rs
// Contains Tauri runtime code

mod handlers;
mod commands;
mod generate_test_data;
mod structs;
mod constants;
mod state;

/// Run Tauri builder to initialize the rust runtime.
pub fn initialize_tauri() {
  generate_test_data::generate();
  tauri::Builder
    ::default()
    .invoke_handler(
      tauri::generate_handler![
        commands::connect_device,
        commands::update_selected_bank,
        commands::update_selected_preset,
        commands::move_message
      ]
    )
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
