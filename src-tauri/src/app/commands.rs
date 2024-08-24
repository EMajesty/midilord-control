// src/commands.rs
// Contains invokable commands for Tauri

use serde_json::Value;
use crate::app::handlers;

/// Command for updating current device config.
#[tauri::command]
pub fn update_device_config(handle: tauri::AppHandle, config: &str) {
  // TODO actual logic
  let object: Value = serde_json::from_str(config).unwrap();
  let device_config = object.to_string();
  handlers::emit_config_update(handle, device_config);
}

/// Command for connecting to the current device.
/// TODO TO BE REMOVED, SHOULD BE DONE AUTOMATICALLY
#[tauri::command]
pub fn connect_device(handle: tauri::AppHandle) {
  // TODO read from device and update global state
  handlers::emit_connection_established(handle);
}

/// Command for switching active bank
#[tauri::command]
pub fn switch_bank(id: u8) {
  println!("Switching bank");
  println!("{:b}", id);
}
