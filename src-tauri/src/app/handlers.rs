// src/handlers.rs
// Contains event handlers for Tauri

use tauri::Manager;

#[derive(Clone, serde::Serialize)]
struct Payload {
    config: String,
}

/// Sends updated device config to the UI.
/// Takes in two parameters, current app handle and stringified JSON value of config.
pub fn emit_config_update(handle: tauri::AppHandle, config: String) {
  handle.emit_all("config_updated", Payload { config }).unwrap();
}

/// Tells the UI that device has been connected.
/// Takes in two parameters, current app handle and stringified JSON value of config.
pub fn emit_connection_established(handle: tauri::AppHandle, config: String) {
  handle.emit_all("device_connected", Payload { config }).unwrap();
}