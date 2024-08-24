// src/app/state.rs
// Contains global state objects to update upon

use std::collections::HashMap;

use once_cell::sync::Lazy;
use super::structs;

pub struct State {
  pub active_bank: u8,
  pub active_preset: u8,
  /// Contains established banks from device, key is bank ID.
  pub banks: Lazy<HashMap<u8, structs::Bank>>,
  /// Contains established presets, key is tuple of (BANK_ID, PRESET_ID).
  pub presets: Lazy<HashMap<(u8, u8), structs::Preset>>,
  /// Contains established messages, key is tuple of (BANK_ID, PRESET_ID).
  pub messages: Lazy<HashMap<(u8, u8), Vec<structs::Message>>>,
}

impl State {
  pub fn insert_bank(&mut self, bank: structs::Bank) {
    self.banks.insert(bank.get_id(), bank);
  }
  pub fn get_active_bank(&self) -> Option<&structs::Bank> {
    return self.banks.get(&self.active_bank);
  }
  pub fn get_banks(&self) -> Vec<structs::Bank> {
    return self.banks.values().cloned().collect();
  }

  pub fn insert_preset(&mut self, bank_id: u8, preset: structs::Preset) {
    self.presets.insert((bank_id, preset.get_id()), preset);
  }
  pub fn get_active_preset(&self) -> Option<&structs::Preset> {
    return self.presets.get(&(self.active_bank, self.active_preset));
  }
  pub fn get_presets(&self) -> Vec<structs::Preset> {
    let mut presets: Vec<structs::Preset> = Vec::new();
    for (key, value) in self.presets.clone().into_iter() {
      if key.0 == self.active_bank {
        presets.push(value);
      }
    }
    return presets;
  }

  pub fn insert_messages(
    &mut self,
    bank_id: u8,
    preset_id: u8,
    messages: Vec<structs::Message>
  ) {
    self.messages.insert((bank_id, preset_id), messages);
  }
  pub fn get_active_messages(&self) -> Option<&Vec<structs::Message>> {
    return self.messages.get(&(self.active_bank, self.active_preset));
  }
}

pub static mut STATE: State = State {
  active_bank: 0,
  active_preset: 0,
  banks: Lazy::new(|| HashMap::new()),
  presets: Lazy::new(|| HashMap::new()),
  messages: Lazy::new(|| HashMap::new()),
};
