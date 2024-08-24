import type { Bank, DeviceConfig, Message, Preset } from "$lib/utils/types";
import { writable } from "svelte/store";

interface Configs {
  config: DeviceConfig;
  banks: Bank[];
  presets: Preset[];
  messages: Message[];
}

const getInitialConfig = (): Configs => {
  const elements = Array.from(new Array(99));
  return {
    config: {
      active_bank: 0,
      active_preset: 0,
    },
    banks: elements.map((_, i) => ({
      id: i,
      name: `Bank ${i + 1}`,
    })),
    presets: Array.from(new Array(8)).map((_, i) => ({
      id: i,
      name: `Preset ${i + 1}`,
    })),
    messages: elements.map((_, i) => ({
      message_action: `Action ${i + 1}`,
      message_type: `Type ${i + 1}`
    }))
  }
}

interface Store extends Configs {
  connected: boolean;
  configLoading: boolean;
}
export const store = writable<Store>({
  connected: false,
  configLoading: false,
  ...getInitialConfig()
});