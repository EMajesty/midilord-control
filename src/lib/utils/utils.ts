import { invoke } from "@tauri-apps/api/tauri";
import type { DeviceConfig } from "./types";

export const updateConfig = (deviceConfig: DeviceConfig) => {
  console.log(deviceConfig);
  invoke("update_device_config", {
    config: JSON.stringify(deviceConfig),
  });
}