export interface Message {
  message_action: string;
  message_type: string;
}

export interface Preset {
  id: number;
  name: string;
}

export interface Bank {
  id: number;
  name: string;
}

export interface DeviceConfig {
  active_bank: number;
  active_preset: number;
}