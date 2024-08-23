// src/app/structs.rs
// Struct definitions for the app

#[derive(Clone, serde::Serialize)]
pub struct Config {
  active_bank: u8,
  active_preset: u8,
}
pub fn construct_config(active_bank: u8, active_preset: u8) -> Config {
  return Config {
    active_bank,
    active_preset,
  }
}

#[derive(Clone, serde::Serialize)]
pub struct Bank {
  id: u8,
  name: String,
}
pub fn construct_bank(id: u8, name: String) -> Bank {
  return Bank {
    id,
    name,
  }
}

#[derive(Clone, serde::Serialize)]
pub struct Preset {
  id: u8,
  name: String,
}
pub fn construct_preset(id: u8, name: String) -> Preset {
  return Preset {
    id,
    name,
  }
}

#[derive(Clone, serde::Serialize)]
pub struct Message {
  message_action: String,
  message_type: String,
}
pub fn construct_message(message_action: String, message_type: String) -> Message {
  return Message {
    message_action,
    message_type,
  }
}