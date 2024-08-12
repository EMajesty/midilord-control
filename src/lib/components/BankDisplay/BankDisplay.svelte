<script lang="ts">
  import type { Bank, DeviceConfig, Preset } from "$lib/utils/types";
  import { beforeUpdate } from "svelte";

  export let deviceConfig: DeviceConfig;
  export let uploadConfig: (config: DeviceConfig) => void;
  let activeBank: Bank;
  let activePreset: Preset;

  function changePreset(presetName: string) {
    uploadConfig({ ...deviceConfig, active_preset: presetName });
  }

  beforeUpdate(() => {
    activeBank =
      deviceConfig.banks.find(
        (bank) => bank.name === deviceConfig.active_bank
      ) ?? deviceConfig.banks[0];

    activePreset =
      activeBank.presets.find(
        (preset) => preset.name === deviceConfig.active_preset
      ) ?? activeBank.presets[0];
  });
</script>

<div class="bank-display">
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
  <div class="message-container">
    {#each activePreset.messages as message, i}
      <div class="message-row">
        <h3>Msg {i + 1}</h3>
        <div class="message-column">
          <b>Action</b>
          <p>{message.action}</p>
        </div>
        <div class="message-column">
          <b>Type</b>
          <p>{message.type}</p>
        </div>
      </div>
    {/each}
  </div>
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
    background-color: var(--background-color-regular);
    flex: 1;
    padding: 5px 8px;
    cursor: pointer;
  }
  li button:hover {
    background-color: var(--background-color-hover);
  }
  li.active button {
    background-color: var(--background-color-dark);
    border-width: 2px 2px 0 2px;
    border-style: solid;
    border-color: var(--background-color-regular);
  }
  .message-container {
    display: flex;
    flex-direction: column;
    gap: 2px;
    background-color: var(--background-color-regular);
    border: 2px solid var(--background-color-regular);
  }
  .message-row {
    display: flex;
    flex-direction: row;
    gap: 5px;
    background-color: var(--background-color-light);
    padding: 3px 5px;
  }
  .message-row h3 {
    margin-block: auto;
  }
  .message-column {
    padding: 3px 5px;
    background-color: var(--background-color-regular);
  }
</style>
