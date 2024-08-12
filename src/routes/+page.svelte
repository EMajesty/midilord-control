<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import type { DeviceConfig } from "$lib/utils/types";
  import BankDisplay from "$lib/components/BankDisplay/BankDisplay.svelte";
  import BankListing from "$lib/components/BankListing/BankListing.svelte";

  let configLoading: Promise<DeviceConfig>;
  const loadConfig = () => {
    configLoading = invoke<string>("read_device_config").then(
      async (config) => {
        // Simulate loading time, remove when done debugging
        await new Promise((res) => setTimeout(res, 2000));
        return JSON.parse(config);
      }
    );
  };
  // For testing purposes, remove when done testing
  const disconnect = () => {
    // @ts-ignore
    configLoading = undefined;
  };

  const uploadConfig = (deviceConfig: DeviceConfig) => {
    configLoading = new Promise((res) => res(deviceConfig));
  };
</script>

<div class="app">
  {#if !configLoading}
    <div class="message-container">
      <h3>Controller not connected</h3>
      <button on:click={loadConfig}>Connect</button>
    </div>
  {:else}
    {#await configLoading}
      <div class="message-container">
        <p>Loading configurations from controller</p>
      </div>
    {:then deviceConfig}
      <div class="content">
        <BankListing {deviceConfig} {uploadConfig} />
        <BankDisplay {deviceConfig} {uploadConfig} />
      </div>
      <button on:click={disconnect}>Disconnect</button>
    {:catch error}
      <div class="message-container">
        <h3>Controller not connected</h3>
        <p>{error.message}</p>
        <button on:click={loadConfig}>Connect</button>
      </div>
    {/await}
  {/if}
</div>

<style>
  .app {
    display: flex;
    flex-direction: column;
    width: 100vw;
    height: 100vh;
  }
  .message-container {
    margin: auto;
    display: flex;
    flex-direction: column;
    gap: var(--whitespace-small);
  }
  .content {
    flex: 1;
    display: flex;
    flex-direction: row;
  }
</style>
