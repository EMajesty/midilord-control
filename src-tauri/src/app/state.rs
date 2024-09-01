// src/app/state.rs
// Contains global state objects to update upon

use std::collections::HashMap;

use once_cell::sync::Lazy;
use super::structs;

#[cfg(test)]
#[path = "./state.test.rs"]
mod tests;

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
  pub fn set_active_bank(&mut self, id: u8) {
    self.active_bank = id;
  }

  pub fn set_active_preset(&mut self, id: u8) {
    self.active_preset = id;
  }

  pub fn insert_bank(&mut self, bank: structs::Bank) {
    self.banks.insert(bank.get_id(), bank);
  }
  pub fn rename_bank(&mut self, name: String) {
    let active_bank = &mut self.active_bank;
    self.banks.insert(
      active_bank.clone(),
      structs::Bank::new(active_bank.clone(), name)
    );
  }
  pub fn get_banks(&self) -> Vec<structs::Bank> {
    let mut banks: Vec<structs::Bank> = self.banks.values().cloned().collect();
    banks.sort_by_key(|a| a.get_id());
    return banks;
  }

  pub fn insert_preset(&mut self, bank_id: u8, preset: structs::Preset) {
    self.presets.insert((bank_id, preset.get_id()), preset);
  }
  pub fn rename_preset(&mut self, name: String) {
    let active_bank = &mut self.active_bank;
    let active_preset = &mut self.active_preset;
    self.presets.insert(
      (active_bank.clone(), active_preset.clone()),
      structs::Preset::new(active_preset.clone(), name)
    );
  }
  pub fn get_presets(&self) -> Vec<structs::Preset> {
    let mut presets: Vec<structs::Preset> = Vec::new();
    for (key, value) in self.presets.clone().into_iter() {
      if key.0 == self.active_bank {
        presets.push(value);
      }
    }
    presets.sort_by_key(|a| a.get_id());
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
  pub fn get_active_messages(&self) -> Vec<structs::Message> {
    let messages = self.messages.get(&(self.active_bank, self.active_preset));
    match messages {
      Some(m) => m.to_vec(),
      None => panic!("No messages found for current preset!"),
    }
  }
  pub fn move_message(&mut self, message_index: u8, target_index: u8) {
    if message_index == target_index {
      return;
    }
    let t_usize: usize = target_index.into();
    let m_usize: usize = message_index.into();
    let messages = &mut self.get_active_messages();
    let message = messages.remove(m_usize);
    let insert_to_index: usize;
    if t_usize == 0 {
      insert_to_index = 0;
    } else if t_usize >= messages.len() {
      insert_to_index = messages.len();
    } else {
      insert_to_index = t_usize;
    }
    messages.insert(insert_to_index, message.clone());
    self.insert_messages(
      self.active_bank,
      self.active_preset,
      messages.to_vec()
    );
  }
}

pub static mut STATE: State = State {
  active_bank: 0,
  active_preset: 0,
  banks: Lazy::new(|| HashMap::new()),
  presets: Lazy::new(|| HashMap::new()),
  messages: Lazy::new(|| HashMap::new()),
};
