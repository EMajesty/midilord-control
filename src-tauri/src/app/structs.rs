// src/app/structs.rs
// Struct definitions for the app

#[derive(Clone, serde::Serialize)]
pub struct Config {
  active_bank: u8,
  active_preset: u8,
}
impl Config {
  pub fn new(active_bank: u8, active_preset: u8) -> Self {
    return Self {
      active_bank,
      active_preset,
    };
  }
}

#[derive(Clone, serde::Serialize)]
pub struct Bank {
  id: u8,
  name: String,
}
impl Bank {
  pub fn get_id(&self) -> u8 {
    self.id
  }
  pub fn get_name(&self) -> String {
    self.name.clone()
  }
  pub fn new(id: u8, name: String) -> Bank {
    return Bank {
      id,
      name,
    };
  }
}

#[derive(Clone, serde::Serialize)]
pub struct Preset {
  id: u8,
  name: String,
}
impl Preset {
  pub fn get_id(&self) -> u8 {
    self.id
  }
  pub fn get_name(&self) -> String {
    self.name.clone()
  }
  pub fn new(id: u8, name: String) -> Preset {
    return Preset {
      id,
      name,
    };
  }
}

#[derive(Clone, serde::Serialize)]
pub struct Message {
  message_action: String,
  message_type: String,
}
impl Message {
  pub fn new(message_action: String, message_type: String) -> Message {
    return Message {
      message_action,
      message_type,
    };
  }
}
