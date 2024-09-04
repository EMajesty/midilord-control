<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import BankDisplay from "$lib/components/BankDisplay/BankDisplay.svelte";
  import BankListing from "$lib/components/BankListing/BankListing.svelte";
  import { onDestroy, onMount } from "svelte";
  import { store } from "../store";
  import {
    getBanksUpdatedListener,
    getBankSwitchedListener,
    getConnectionListener,
    getMessagesEditedListener,
    getPresetsUpdatedListener,
    getPresetSwitchedListener,
  } from "$lib/utils/utils";
  import { get } from "svelte/store";

  let connected: boolean;
  let configLoading = get(store).configLoading;
  const unsubscribe = store.subscribe((value) => {
    connected = value.connected;
    configLoading = value.configLoading;
  });

  const connect = async () => {
    configLoading = true;
    await new Promise((res) => setTimeout(res, 100));
    invoke("connect_device");
  };

  // For testing purposes, remove when done testing
  const disconnect = () => {
    store.update((value) => ({ ...value, connected: false }));
  };

  onMount(() => {
    const connectionListener = getConnectionListener();
    const bankSwitchedListener = getBankSwitchedListener();
    const presetSwitchedListener = getPresetSwitchedListener();
    const messageMovedListener = getMessagesEditedListener();
    const banksUpdatedListener = getBanksUpdatedListener();
    const presetsUpdatedListener = getPresetsUpdatedListener();
    return () => {
      connectionListener.then((unlisten) => unlisten());
      bankSwitchedListener.then((unlisten) => unlisten());
      presetSwitchedListener.then((unlisten) => unlisten());
      messageMovedListener.then((unlisten) => unlisten());
      banksUpdatedListener.then((unlisten) => unlisten());
      presetsUpdatedListener.then((unlisten) => unlisten());
    };
  });

  onDestroy(unsubscribe);
</script>

<div class="app">
  {#if !connected && !configLoading}
    <div class="message-container">
      <div class="message-wrapper">
        <h3>Controller not connected</h3>
        <button on:click={connect}>Connect</button>
      </div>
    </div>
  {:else if configLoading}
    <div class="message-container">
      <div class="message-wrapper">
        <p>Loading configurations from controller</p>
      </div>
    </div>
  {/if}
  <div class="content">
    <BankListing />
    <BankDisplay />
  </div>
  <button on:click={disconnect} class="disconnect-button">Disconnect</button>
</div>

<style>
  .app {
    display: flex;
    flex-direction: column;
    width: 100vw;
    height: 100vh;
    background-color: var(--gray-2);
    overflow: hidden;
  }
  .message-container {
    display: flex;
    flex-direction: column;
    position: fixed;
    top: 0;
    left: 0;
    z-index: 5;
    height: 100%;
    width: 100%;
    background: #0729ec41;
    backdrop-filter: blur(7.5px);
    -webkit-backdrop-filter: blur(7.5px);
  }
  .message-wrapper {
    margin: auto;
    display: flex;
    gap: var(--whitespace-large);
    flex-direction: column;
    padding: var(--whitespace-large);
    border: 5px outset var(--blue-5);
    background-color: var(--blue-4);
    color: var(--white-blue);
  }
  .content {
    flex: 1;
    display: flex;
    flex-direction: row;
  }
  .disconnect-button {
    position: absolute;
    bottom: 10px;
    right: 10px;
  }
</style>
