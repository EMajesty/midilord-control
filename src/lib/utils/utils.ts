import { invoke } from "@tauri-apps/api/tauri";
import type { DeviceConfig } from "./types";

export const updateConfig = (deviceConfig: DeviceConfig) => {
  invoke("update_device_config", {
    config: JSON.stringify(deviceConfig),
  });
}