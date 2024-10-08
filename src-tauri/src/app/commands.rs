// src/commands.rs
// Contains invokable commands for Tauri

use crate::app::handlers;
use super::{ state::STATE, structs };

/// Command for connecting to the current device.
/// TODO TO BE REMOVED, SHOULD BE DONE AUTOMATICALLY
#[tauri::command]
pub fn connect_device(handle: tauri::AppHandle) {
  // TODO read from device and update global state
  handlers::emit_connection_established(handle);
}

/// Command for switching active bank
#[tauri::command]
pub fn update_selected_bank(handle: tauri::AppHandle, id: u8) {
  unsafe {
    STATE.set_active_bank(id);
    STATE.set_active_preset(0);
    handlers::emit_bank_switched(handle);
  }
}

/// Command for switching active preset
#[tauri::command]
pub fn update_selected_preset(handle: tauri::AppHandle, id: u8) {
  unsafe {
    STATE.set_active_preset(id);
    handlers::emit_preset_switched(handle);
  }
}

/// Command for moving messages to another index
#[tauri::command]
pub fn move_message(
  handle: tauri::AppHandle,
  message_index: u8,
  target_index: u8
) {
  unsafe {
    STATE.move_message(message_index, target_index);
    handlers::emit_messages_edited(handle)
  }
}

/// Command for editing message
#[tauri::command]
pub fn edit_message(
  handle: tauri::AppHandle,
  message_index: u8,
  message: structs::Message
) {
  unsafe {
    STATE.edit_message(message_index, message);
    handlers::emit_messages_edited(handle)
  }
}

/// Command for renaming active bank
#[tauri::command]
pub fn rename_bank(handle: tauri::AppHandle, name: String) {
  unsafe {
    STATE.rename_bank(name);
    handlers::emit_banks_updated(handle);
  }
}

/// Command for renaming active preset
#[tauri::command]
pub fn rename_preset(handle: tauri::AppHandle, name: String) {
  unsafe {
    STATE.rename_preset(name);
    handlers::emit_presets_updated(handle);
  }
}
