// src/app/generate_test_data.rs
// Generate data for us Windows scrubs

use super::state::STATE;
use super::structs;
use super::constants;

fn generate_struct_vector<T>(length: usize, callback: fn(u8) -> T) -> Vec<T> {
  let mut index = 0;
  return std::iter
    ::repeat_with(|| {
      let resource = callback(index);
      index += 1;
      return resource;
    })
    .take(length)
    .collect::<Vec<_>>();
}

fn generate_banks() -> Vec<structs::Bank> {
  return generate_struct_vector(constants::BANK_COUNT, |index|
    structs::Bank::new(index, format!("Bank {}", index + 1))
  );
}

fn generate_presets() -> Vec<structs::Preset> {
  return generate_struct_vector(constants::PRESET_COUNT, |index|
    structs::Preset::new(index, format!("Preset {}", index + 1))
  );
}

fn generate_messages() -> Vec<structs::Message> {
  return generate_struct_vector(constants::MESSAGE_COUNT, |index|
    structs::Message::new(
      format!("Action {}", index + 1),
      format!("Type {}", index + 1)
    )
  );
}

/// Sets initial values to the global state.
pub fn generate() {
  unsafe {
    let banks = generate_banks();
    for bank in banks {
      let bank_id = bank.get_id();
      STATE.insert_bank(bank);
      let presets = generate_presets();
      for preset in presets {
        let preset_id = preset.get_id();
        STATE.insert_preset(bank_id, preset);
        STATE.insert_messages(bank_id, preset_id, generate_messages());
      }
    }
  }
}
