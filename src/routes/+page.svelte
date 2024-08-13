<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { listen } from "@tauri-apps/api/event";
  import type { DeviceConfig } from "$lib/utils/types";
  import BankDisplay from "$lib/components/BankDisplay/BankDisplay.svelte";
  import BankListing from "$lib/components/BankListing/BankListing.svelte";
  import { onDestroy, onMount } from "svelte";
  import { store } from "../store";

  let deviceConfig: DeviceConfig | undefined;
  let connected: boolean;
  const unsubscribe = store.subscribe((value) => {
    deviceConfig = value.deviceConfig;
    connected = value.connected;
  });
  let configLoading = false;

  const connect = () => {
    configLoading = true;
    invoke("connect_device");
  };

  // For testing purposes, remove when done testing
  const disconnect = () => {
    store.update((value) => ({ ...value, connected: false }));
  };

  onMount(() => {
    const connectionListener = listen<{ config: string }>(
      "device_connected",
      (event) => {
        store.update((value) => ({
          ...value,
          connected: true,
          deviceConfig: JSON.parse(event.payload.config),
        }));
        configLoading = false;
      }
    );
    const updateListener = listen<{ config: string }>(
      "config_updated",
      (event) => {
        store.update((value) => ({
          ...value,
          deviceConfig: JSON.parse(event.payload.config),
        }));
        configLoading = false;
      }
    );
    return () => {
      connectionListener.then((unlisten) => unlisten());
      updateListener.then((unlisten) => unlisten());
    };
  });

  onDestroy(unsubscribe);
</script>

<div class="app">
  {#if !connected || !deviceConfig}
    <div class="message-container">
      <h3>Controller not connected</h3>
      <button on:click={connect}>Connect</button>
    </div>
  {:else if configLoading}
    <div class="message-container">
      <p>Loading configurations from controller</p>
    </div>
  {:else if deviceConfig}
    <div class="content">
      <BankListing />
      <BankDisplay />
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
