// src/handlers.rs
// Contains event handlers for Tauri

use tauri::Manager;

use super::state::STATE;
use super::structs;
use super::generate_test_data;

#[derive(Clone, serde::Serialize)]
struct TestPayload {
  config: String,
  conf: structs::Config,
  banks: Vec<structs::Bank>,
  presets: Vec<structs::Preset>,
  messages: Vec<structs::Message>,
}

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
  unsafe {
    let messages = STATE.get_active_messages();
    match messages {
      Some(m) => {
        handle
          .emit_all("device_connected", TestPayload {
            config,
            conf: structs::Config::new(STATE.active_bank, STATE.active_preset),
            banks: STATE.get_banks(),
            presets: STATE.get_presets(),
            messages: m.clone(),
          })
          .unwrap();
      }
      None => { panic!("No messages created for current preset!") }
    }
  }
}
