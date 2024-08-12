<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { listen } from "@tauri-apps/api/event";
  import type { DeviceConfig } from "$lib/utils/types";
  import BankDisplay from "$lib/components/BankDisplay/BankDisplay.svelte";
  import BankListing from "$lib/components/BankListing/BankListing.svelte";
  import {  onMount } from "svelte";

  let configLoading = false;
  let deviceConfig: DeviceConfig | undefined = undefined;
  const connect = () => {
    configLoading = true;
    invoke("connect_device");
  };

  onMount(() => {
    const connectionListener = listen<{ config: string }>(
      "device_connected",
      (event) => {
        deviceConfig = JSON.parse(event.payload.config);
        configLoading = false;
      }
    );
    const updateListener = listen<{ config: string }>(
      "config_updated",
      (event) => {
        deviceConfig = JSON.parse(event.payload.config);
        configLoading = false;
      }
    );
    return () => {
      connectionListener.then((unlisten) => unlisten());
      updateListener.then((unlisten) => unlisten());
    };
  });

  // For testing purposes, remove when done testing
  const disconnect = () => {
    deviceConfig = undefined;
  };

  const uploadConfig = (deviceConfig: DeviceConfig) => {
    invoke("update_device_config", {
      config: JSON.stringify(deviceConfig),
    });
  };
</script>

<div class="app">
  {#if !configLoading && !deviceConfig}
    <div class="message-container">
      <h3>Controller not connected</h3>
      <button on:click={connect}>Connect</button>
    </div>
  {:else if !deviceConfig}
    <div class="message-container">
      <p>Loading configurations from controller</p>
    </div>
  {:else}
    <div class="content">
      <BankListing {deviceConfig} {uploadConfig} />
      <BankDisplay {deviceConfig} {uploadConfig} />
    </div>
    <button on:click={disconnect}>Disconnect</button>
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
