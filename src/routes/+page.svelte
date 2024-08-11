<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import type { Bank, DeviceConfig, Preset } from "$lib/utils/types";
  import BankDisplay from "$lib/components/BankDisplay/BankDisplay.svelte";

  let configLoading: Promise<DeviceConfig> = invoke<string>(
    "read_device_config"
  )
    .then((config) => JSON.parse(config));
</script>

<div class="container">
  {#await configLoading}
    <p>Loading configurations from controller</p>
  {:then config}
    <BankDisplay deviceConfig={config} />
  {:catch error}
    <p>{error.message}</p>
  {/await}
</div>

<style>
</style>
