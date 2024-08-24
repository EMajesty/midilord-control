import { invoke } from "@tauri-apps/api/tauri";
import type { Bank, DeviceConfig, Message, Preset } from "./types";
import { listen } from "@tauri-apps/api/event";
import { store, type Store } from "../../store";

export const selectBank = (id: number) => {
  invoke("update_selected_bank", {
    id: id.toUint8(),
  });
}

export const selectPreset = (id: number) => {
  invoke("update_selected_preset", {
    id: id.toUint8(),
  });
}

export const moveMessage = (messageIndex: number, targetIndex: number) => {
  invoke("move_message", {
    messageIndex: messageIndex.toUint8(),
    targetIndex: targetIndex.toUint8()
  })
}

const getUpdateStoreCallback = (staticValues?: Partial<Store>) => ({ payload }: { payload: Partial<Store> }) => {
  store.update((value) => ({
    ...value,
    ...payload,
    ...staticValues
  }));
}

interface ConnectionPayload {
  config: DeviceConfig;
  banks: Bank[];
  presets: Preset[];
  messages: Message[];
}
export const getConnectionListener = () => {
  return listen<ConnectionPayload>(
    "device_connected",
    getUpdateStoreCallback({
      connected: true,
      configLoading: false,
    })
  );
}

interface BankPayload {
  config: DeviceConfig;
  presets: Preset[];
  messages: Message[];
}
export const getBankSwitchedListener = () => {
  return listen<BankPayload>(
    "bank_switched",
    getUpdateStoreCallback(),
  );
}

interface PresetPayload {
  config: DeviceConfig;
  messages: Message[];
}
export const getPresetSwitchedListener = () => {
  return listen<PresetPayload>(
    "preset_switched",
    getUpdateStoreCallback(),
  );
}

interface MessagePayload {
  messages: Message[];
}
export const getMessageMovedListener = () => {
  return listen<MessagePayload>(
    "message_moved",
    getUpdateStoreCallback(),
  );
}