export interface Message {
  action: string;
  type: string;
}

export interface Preset {
  name: string;
  messages: Message[];
}

export interface Bank {
  name: string;
  active_preset: string;
  presets: Preset[];
}

export interface DeviceConfig {
  active_bank: string;
  banks: Bank[];
}