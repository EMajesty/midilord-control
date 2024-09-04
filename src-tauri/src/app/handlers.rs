// src/handlers.rs
// Contains event handlers for Tauri

use tauri::Manager;

use super::state::STATE;
use super::structs;

#[derive(Clone, serde::Serialize)]
struct ConfigPayload {
  config: structs::Config,
  banks: Vec<structs::Bank>,
  presets: Vec<structs::Preset>,
  messages: Vec<structs::Message>,
}

/// Tells the UI that device has been connected.
pub fn emit_connection_established(handle: tauri::AppHandle) {
  unsafe {
    handle
      .emit_all("device_connected", ConfigPayload {
        config: structs::Config::new(STATE.active_bank, STATE.active_preset),
        banks: STATE.get_banks(),
        presets: STATE.get_presets(),
        messages: STATE.get_active_messages(),
      })
      .unwrap();
  }
}

#[derive(Clone, serde::Serialize)]
struct BankPayload {
  config: structs::Config,
  presets: Vec<structs::Preset>,
  messages: Vec<structs::Message>,
}

/// Sends selected bank details to the UI.
pub fn emit_bank_switched(handle: tauri::AppHandle) {
  unsafe {
    handle
      .emit_all("bank_switched", BankPayload {
        config: structs::Config::new(STATE.active_bank, STATE.active_preset),
        presets: STATE.get_presets(),
        messages: STATE.get_active_messages(),
      })
      .unwrap();
  }
}

#[derive(Clone, serde::Serialize)]
struct PresetPayload {
  config: structs::Config,
  messages: Vec<structs::Message>,
}

/// Sends selected preset details to the UI.
pub fn emit_preset_switched(handle: tauri::AppHandle) {
  unsafe {
    handle
      .emit_all("preset_switched", PresetPayload {
        config: structs::Config::new(STATE.active_bank, STATE.active_preset),
        messages: STATE.get_active_messages(),
      })
      .unwrap();
  }
}

#[derive(Clone, serde::Serialize)]
struct MessagePayload {
  messages: Vec<structs::Message>,
}
/// Sends selected preset details to the UI.
pub fn emit_messages_edited(handle: tauri::AppHandle) {
  unsafe {
    handle
      .emit_all("messages_edited", MessagePayload {
        messages: STATE.get_active_messages(),
      })
      .unwrap();
  }
}

#[derive(Clone, serde::Serialize)]
struct BankUpdatePayload {
  banks: Vec<structs::Bank>,
}
/// Sends updated list of banks on update to the UI.
pub fn emit_banks_updated(handle: tauri::AppHandle) {
  unsafe {
    handle
      .emit_all("banks_updated", BankUpdatePayload {
        banks: STATE.get_banks(),
      })
      .unwrap();
  }
}

#[derive(Clone, serde::Serialize)]
struct PresetUpdatePayload {
  presets: Vec<structs::Preset>,
}
/// Sends updated list of banks on update to the UI.
pub fn emit_presets_updated(handle: tauri::AppHandle) {
  unsafe {
    handle
      .emit_all("presets_updated", PresetUpdatePayload {
        presets: STATE.get_presets(),
      })
      .unwrap();
  }
}
