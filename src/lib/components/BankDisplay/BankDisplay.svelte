<script lang="ts">
  import { updateConfig } from "$lib/utils/utils";
  import { onDestroy } from "svelte";
  import MessageList from "./MessageList.svelte";
  import { store } from "../../../store";
  import { get } from "svelte/store";

  let deviceConfig = get(store).deviceConfig;
  let activeBank = deviceConfig?.banks.find(
    (bank) => bank.name === deviceConfig?.active_bank
  );
  const unsubscribe = store.subscribe((value) => {
    deviceConfig = value.deviceConfig;
    activeBank = deviceConfig?.banks.find(
      (bank) => bank.name === deviceConfig?.active_bank
    );
  });

  function changePreset(presetName: string) {
    if (!deviceConfig) return;
    updateConfig({ ...deviceConfig, active_preset: presetName });
  }

  onDestroy(unsubscribe);
</script>

<div class="bank-display">
  {#if activeBank && deviceConfig}
    <h1>{activeBank.name}</h1>
    <nav>
      <ul>
        {#each activeBank.presets as preset}
          <li
            class={preset.name === deviceConfig.active_preset
              ? "active"
              : undefined}
          >
            <button on:click={() => changePreset(preset.name)}
              >{preset.name}</button
            >
          </li>
        {/each}
      </ul>
    </nav>
    <MessageList />
  {/if}
</div>

<style>
  .bank-display {
    flex: 1;
    padding: var(--whitespace-large);
  }
  ul {
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: row;
    gap: 2px;
  }
  li {
    list-style: none;
    background-color: var(--background-color-regular);
    display: flex;
  }
  li button {
    font-weight: bold;
    border: none;
    background-color: var(--background-color-dark);
    flex: 1;
    padding: 5px 8px;
    cursor: pointer;
  }
  li button:hover {
    background-color: var(--background-color-hover);
  }
  li.active button {
    background-color: var(--background-color-regular);
  }
</style>
