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

#[cfg(test)]
mod tests {
  use super::*;
  mod state {
    use serial_test::serial;

    use super::*;

    fn setup() {
      unsafe {
        STATE.active_bank = 0;
        STATE.active_preset = 0;
        STATE.banks.clear();
        STATE.presets.clear();
        STATE.messages.clear();
      }
    }

    #[test]
    #[serial]
    fn sets_active_bank_to_state() {
      setup();
      unsafe {
        STATE.set_active_bank(1);
        assert_eq!(STATE.active_bank, 1);
      }
    }

    #[test]
    #[serial]
    fn sets_active_preset_to_state() {
      setup();
      unsafe {
        STATE.set_active_preset(1);
        assert_eq!(STATE.active_preset, 1);
      }
    }

    #[test]
    #[serial]
    fn inserts_bank_to_state() {
      setup();
      unsafe {
        let id: u8 = 1;
        STATE.insert_bank(structs::Bank::new(id, "Test bank".to_string()));
        let inserted = STATE.banks.get(&id);
        match inserted {
          Some(b) => {
            assert_eq!(b.get_name(), "Test bank");
          }
          None => panic!("Bank was not inserted correctly!"),
        }
      }
    }

    #[test]
    #[serial]
    fn renames_active_bank_in_state() {
      setup();
      unsafe {
        let id: u8 = 1;
        let original_name = "Test bank".to_string();
        let new_name = "Renamed bank".to_string();
        STATE.insert_bank(structs::Bank::new(id, original_name));
        STATE.set_active_bank(id);
        STATE.rename_bank(new_name.clone());
        let bank = STATE.banks.get(&id);
        match bank {
          Some(b) => {
            let bank_id = b.get_id();
            let name = b.get_name();
            assert_eq!(id, bank_id, "Expected {}, got {}", bank_id, id);
            assert_eq!(name, new_name, "Expected {}, got {}", new_name, name);
          }
          None => panic!("Bank was not inserted correctly!"),
        }
      }
    }

    #[test]
    #[serial]
    fn returns_sorted_banks_as_vector() {
      setup();
      unsafe {
        let ids: [u8; 5] = [1, 3, 2, 5, 4];
        let name = "Bank".to_string();
        for id in ids {
          STATE.insert_bank(structs::Bank::new(id, name.clone()));
        }
        let banks = STATE.get_banks();
        let sorted_ids: Vec<u8> = banks
          .iter()
          .map(|bank| bank.get_id())
          .collect();

        assert_eq!(
          sorted_ids,
          [1, 2, 3, 4, 5].to_vec(),
          "Banks were not sorted correctly, got {sorted_ids:?}"
        )
      }
    }

    #[test]
    #[serial]
    fn inserts_preset_with_correct_key() {
      setup();
      unsafe {
        let bank_id: u8 = 1;
        let preset_id: u8 = 1;
        let preset_name = "Preset name".to_string();
        STATE.insert_preset(
          bank_id,
          structs::Preset::new(preset_id.clone(), preset_name.clone())
        );
        let inserted = STATE.presets.get(&(bank_id, preset_id));
        match inserted {
          Some(p) => {
            let id = p.get_id();
            let name = p.get_name();
            assert_eq!(preset_id, id, "Expected {}, got {}", preset_id, id);
            assert_eq!(
              preset_name,
              name,
              "Expected {}, got {}",
              preset_name,
              name
            )
          }
          None => panic!("Preset was not inserted correctly!"),
        }
      }
    }

    #[test]
    #[serial]
    fn renames_active_preset_in_state() {
      setup();
      unsafe {
        let bank_id: u8 = 1;
        let id: u8 = 1;
        let original_name = "Test preset".to_string();
        let new_name = "Renamed preset".to_string();
        STATE.insert_preset(
          bank_id.clone(),
          structs::Preset::new(id, original_name)
        );
        STATE.set_active_bank(bank_id);
        STATE.set_active_preset(id);
        STATE.rename_preset(new_name.clone());
        let preset = STATE.presets.get(&(bank_id, id));
        match preset {
          Some(p) => {
            let preset_id = p.get_id();
            let name = p.get_name();
            assert_eq!(id, preset_id, "Expected {}, got {}", preset_id, id);
            assert_eq!(name, new_name, "Expected {}, got {}", new_name, name);
          }
          None => panic!("Preset was not inserted correctly!"),
        }
      }
    }

    #[test]
    #[serial]
    fn gets_presets_of_active_bank_as_sorted() {
      setup();
      unsafe {
        let bank_id: u8 = 1;
        STATE.set_active_bank(bank_id);
        let ids: [u8; 5] = [1, 3, 2, 5, 4];
        let name = "Preset".to_string();
        for id in ids {
          STATE.insert_preset(bank_id, structs::Preset::new(id, name.clone()));
        }
        let presets = STATE.get_presets();
        let sorted_ids: Vec<u8> = presets
          .iter()
          .map(|preset| preset.get_id())
          .collect();

        assert_eq!(
          sorted_ids,
          [1, 2, 3, 4, 5].to_vec(),
          "Presets were not sorted correctly, got {sorted_ids:?}"
        )
      }
    }

    #[test]
    #[serial]
    fn inserts_messages_to_state() {
      setup();
      unsafe {
        let bank_id: u8 = 1;
        let preset_id: u8 = 1;
        let mut messages: Vec<structs::Message> = Vec::new();
        messages.push(
          structs::Message::new("Action".to_string(), "Type".to_string())
        );
        STATE.insert_messages(bank_id, preset_id, messages.clone());
        let state_messages = STATE.messages.get(&(bank_id, preset_id));
        match state_messages {
          Some(m) => {
            assert_eq!(
              messages.len(),
              m.len(),
              "Invalid input inserted into state!"
            );
          }
          None => panic!("Messages were not inserted correctly!"),
        }
      }
    }

    #[test]
    #[should_panic(expected = "No messages found for current preset!")]
    #[serial]
    fn panics_if_no_messages_for_preset() {
      setup();
      unsafe {
        let bank_id: u8 = 1;
        let preset_id: u8 = 1;
        STATE.active_bank = bank_id;
        STATE.active_preset = preset_id;
        STATE.get_active_messages();
      }
    }

    #[test]
    #[serial]
    fn gets_messages_for_active_preset() {
      setup();
      unsafe {
        let bank_id: u8 = 1;
        let preset_id: u8 = 1;
        STATE.active_bank = bank_id;
        STATE.active_preset = preset_id;
        let mut messages: Vec<structs::Message> = Vec::new();
        messages.push(
          structs::Message::new("Action".to_string(), "Type".to_string())
        );
        STATE.insert_messages(bank_id, preset_id, messages.clone());
        let active_messages = STATE.get_active_messages();
        assert_eq!(
          messages.len(),
          active_messages.len(),
          "Invalid input inserted into state!"
        )
      }
    }

    fn setup_messages() {
      unsafe {
        let bank_id: u8 = 1;
        let preset_id: u8 = 1;
        STATE.active_bank = bank_id;
        STATE.active_preset = preset_id;
        let indices = [0, 1, 2, 3, 4];
        let mut messages: Vec<structs::Message> = Vec::new();
        for i in indices {
          messages.push(
            structs::Message::new(format!("{}", i), "Type".to_string())
          );
        }
        STATE.insert_messages(bank_id, preset_id, messages.clone());
      }
    }

    #[test]
    #[serial]
    fn moves_message_to_after() {
      setup();
      setup_messages();
      unsafe {
        STATE.move_message(1, 2);
        let actions: Vec<String> = STATE.get_active_messages()
          .iter()
          .map(|message| message.get_message_action())
          .collect();
        assert_eq!(
          actions,
          ["0", "2", "1", "3", "4"],
          "Moving message was not succesful!"
        );
      }
    }

    #[test]
    #[serial]
    fn moves_message_to_before() {
      setup();
      setup_messages();
      unsafe {
        STATE.move_message(2, 1);
        let actions: Vec<String> = STATE.get_active_messages()
          .iter()
          .map(|message| message.get_message_action())
          .collect();
        assert_eq!(
          actions,
          ["0", "2", "1", "3", "4"],
          "Moving message was not succesful!"
        );
      }
    }

    #[test]
    #[serial]
    fn moves_message_to_start() {
      setup();
      setup_messages();
      unsafe {
        STATE.move_message(2, 0);
        let actions: Vec<String> = STATE.get_active_messages()
          .iter()
          .map(|message| message.get_message_action())
          .collect();
        assert_eq!(
          actions,
          ["2", "0", "1", "3", "4"],
          "Moving message was not succesful!"
        );
      }
    }

    #[test]
    #[serial]
    fn moves_message_to_end() {
      setup();
      setup_messages();
      unsafe {
        STATE.move_message(2, 4);
        let actions: Vec<String> = STATE.get_active_messages()
          .iter()
          .map(|message| message.get_message_action())
          .collect();
        assert_eq!(
          actions,
          ["0", "1", "3", "4", "2"],
          "Moving message was not succesful!"
        );
      }
    }

    #[test]
    #[serial]
    fn swaps_first_two() {
      setup();
      setup_messages();
      unsafe {
        STATE.move_message(0, 1);
        let actions: Vec<String> = STATE.get_active_messages()
          .iter()
          .map(|message| message.get_message_action())
          .collect();
        assert_eq!(
          actions,
          ["1", "0", "2", "3", "4"],
          "Moving message was not succesful!"
        );
      }
    }

    #[test]
    #[serial]
    fn swaps_last_two() {
      setup();
      setup_messages();
      unsafe {
        STATE.move_message(3, 4);
        let actions: Vec<String> = STATE.get_active_messages()
          .iter()
          .map(|message| message.get_message_action())
          .collect();
        assert_eq!(
          actions,
          ["0", "1", "2", "4", "3"],
          "Moving message was not succesful!"
        );
      }
    }
  }
}
