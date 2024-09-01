#[cfg(test)]
mod tests {
  use serial_test::serial;

  use crate::app::{ state::STATE, structs };

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
        structs::Message::new(structs::MessageType::EMPTY, 0, 0, 0)
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
        structs::Message::new(structs::MessageType::EMPTY, 0, 0, 0)
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
      let types = [
        structs::MessageType::EMPTY,
        structs::MessageType::INTERNAL,
        structs::MessageType::CONTROL,
        structs::MessageType::PROGRAM,
      ];
      let mut messages: Vec<structs::Message> = Vec::new();
      for mtype in types {
        messages.push(structs::Message::new(mtype, 0, 0, 0));
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
      let actions: Vec<structs::MessageType> = STATE.get_active_messages()
        .iter()
        .map(|message| message.get_message_type())
        .collect();
      assert_eq!(
        actions,
        [
          structs::MessageType::EMPTY,
          structs::MessageType::CONTROL,
          structs::MessageType::INTERNAL,
          structs::MessageType::PROGRAM,
        ],
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
      let actions: Vec<structs::MessageType> = STATE.get_active_messages()
        .iter()
        .map(|message| message.get_message_type())
        .collect();
      assert_eq!(
        actions,
        [
          structs::MessageType::EMPTY,
          structs::MessageType::CONTROL,
          structs::MessageType::INTERNAL,
          structs::MessageType::PROGRAM,
        ],
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
      let actions: Vec<structs::MessageType> = STATE.get_active_messages()
        .iter()
        .map(|message| message.get_message_type())
        .collect();
      assert_eq!(
        actions,
        [
          structs::MessageType::CONTROL,
          structs::MessageType::EMPTY,
          structs::MessageType::INTERNAL,
          structs::MessageType::PROGRAM,
        ],
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
      let actions: Vec<structs::MessageType> = STATE.get_active_messages()
        .iter()
        .map(|message| message.get_message_type())
        .collect();
      assert_eq!(
        actions,
        [
          structs::MessageType::EMPTY,
          structs::MessageType::INTERNAL,
          structs::MessageType::PROGRAM,
          structs::MessageType::CONTROL,
        ],
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
      let actions: Vec<structs::MessageType> = STATE.get_active_messages()
        .iter()
        .map(|message| message.get_message_type())
        .collect();
      assert_eq!(
        actions,
        [
          structs::MessageType::INTERNAL,
          structs::MessageType::EMPTY,
          structs::MessageType::CONTROL,
          structs::MessageType::PROGRAM,
        ],
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
      STATE.move_message(2, 3);
      let actions: Vec<structs::MessageType> = STATE.get_active_messages()
        .iter()
        .map(|message| message.get_message_type())
        .collect();
      assert_eq!(
        actions,
        [
          structs::MessageType::EMPTY,
          structs::MessageType::INTERNAL,
          structs::MessageType::PROGRAM,
          structs::MessageType::CONTROL,
        ],
        "Moving message was not succesful!"
      );
    }
  }

  #[test]
  #[serial]
  fn edits_message_type() {
    setup();
    setup_messages();
    unsafe {
      STATE.edit_message(
        0,
        structs::Message::new(structs::MessageType::PROGRAM, 0, 0, 0)
      );
      let actions: Vec<structs::MessageType> = STATE.get_active_messages()
        .iter()
        .map(|message| message.get_message_type())
        .collect();
      assert_eq!(
        actions,
        [
          structs::MessageType::PROGRAM,
          structs::MessageType::INTERNAL,
          structs::MessageType::CONTROL,
          structs::MessageType::PROGRAM,
        ],
        "Editing message was not succesful!"
      );
    }
  }
}
