import type { DeviceConfig } from "$lib/utils/types";
import { writable } from "svelte/store";

interface Store {
  connected: boolean;
  deviceConfig?: DeviceConfig;
}
export const store = writable<Store>({
  connected: false,
  deviceConfig: undefined
});