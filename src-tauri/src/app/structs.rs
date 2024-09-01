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

#[derive(Clone, serde::Serialize, Debug, PartialEq)]
pub enum MessageType {
  EMPTY,
  INTERNAL,
  CONTROL,
  PROGRAM,
}

#[derive(Clone, serde::Serialize)]
pub struct Message {
  message_type: MessageType,
  message_channel: u8,
  message_number: u8,
  message_value: u8,
}
impl Message {
  pub fn new(
    message_type: MessageType,
    message_channel: u8,
    message_number: u8,
    message_value: u8
  ) -> Message {
    return Message {
      message_type,
      message_channel,
      message_number,
      message_value,
    };
  }

  pub fn get_message_type(&self) -> MessageType {
    self.message_type.clone()
  }
}
