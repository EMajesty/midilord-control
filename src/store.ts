import type { DeviceConfig } from "$lib/utils/types";
import { writable } from "svelte/store";

const getInitialConfig = ():DeviceConfig => {
  const elements = Array.from(new Array(99));
  return {
    active_bank: "Bank 1",
    active_preset: "Preset 1",
    banks: elements.map((_,i) => ({
      name: `Bank ${i + 1}`,
      presets: Array.from(new Array(8)).map((_,i) => ({
        name: `Preset ${i + 1}`,
        messages: elements.map(() => ({
          action: "Action",
          type: "Type"
        }))
      }))
    }))
  }
}

interface Store {
  connected: boolean;
  deviceConfig: DeviceConfig;
}
export const store = writable<Store>({
  connected: false,
  deviceConfig: getInitialConfig()
});