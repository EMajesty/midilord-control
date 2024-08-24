import { invoke } from "@tauri-apps/api/tauri";
import type { Bank, DeviceConfig, Message, Preset } from "./types";
import { listen } from "@tauri-apps/api/event";
import { store } from "../../store";

export const updateConfig = (deviceConfig: DeviceConfig) => {
  invoke("update_device_config", {
    config: JSON.stringify(deviceConfig),
  });
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
    (event) => {
      store.update((value) => ({
        ...value,
        ...event.payload,
        connected: true,
        configLoading: false,
      }));
    },
  );
}