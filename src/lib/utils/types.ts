export enum MessageType {
  EMPTY = "EMPTY",
  INTERNAL = "INTERNAL",
  CONTROL = "CONTROL",
  PROGRAM = "PROGRAM",
}

type Enumerate<N extends number, Acc extends number[] = []> = Acc['length'] extends N
  ? Acc[number]
  : Enumerate<N, [...Acc, Acc['length']]>;

type Range<F extends number, T extends number> = Exclude<Enumerate<T>, Enumerate<F>>;

export interface Message {
  message_type: MessageType;
  message_channel: Range<1, 16>;
  message_number: Range<0, 127>;
  message_value: Range<0, 127>;
}

export interface Preset {
  id: Range<0, 7>;
  name: string;
}

export interface Bank {
  id: Range<0, 29>;
  name: string;
}

export interface DeviceConfig {
  active_bank: Range<0, 29>;
  active_preset: Range<0, 7>;
}