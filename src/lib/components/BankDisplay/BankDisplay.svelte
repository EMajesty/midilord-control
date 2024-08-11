<script lang="ts">
  import type { DeviceConfig } from "$lib/utils/types";

  export let deviceConfig: DeviceConfig;
  let activeBank =
    deviceConfig.banks.find((bank) => bank.name === deviceConfig.active_bank) ??
    deviceConfig.banks[0];

  let activePreset =
    activeBank.presets.find(
      (preset) => preset.name === activeBank.active_preset
    ) ?? activeBank.presets[0];

  function swapPreset(presetName: string) {
    // TODO call controller instead and reload
    activeBank.active_preset = presetName;
    activePreset =
      activeBank.presets.find((preset) => preset.name === presetName) ??
      activeBank.presets[0];
  }
</script>

<h1>{activeBank.name}</h1>
<nav>
  <ul>
    {#each activeBank.presets as preset}
      <li
        class={preset.name === activeBank.active_preset ? "active" : undefined}
      >
        <button on:click={() => swapPreset(preset.name)}>{preset.name}</button>
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

<style>
  * {
    box-sizing: border-box;
  }
  p,
  h1,
  h3 {
    margin: 0;
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
    background-color: #ccc;
    display: flex;
  }
  li button {
    font-weight: bold;
    border: none;
    background-color: #ccc;
    flex: 1;
    padding: 5px 8px;
    cursor: pointer;
  }
  li button:hover {
    background-color: #bbb;
  }
  li.active button {
    background-color: #999;
    border-width: 2px 2px 0 2px;
    border-style: solid;
    border-color: #ccc;
  }
  .message-container {
    display: flex;
    flex-direction: column;
    gap: 2px;
    background-color: #ccc;
    border: 2px solid #ccc;
  }
  .message-row {
    display: flex;
    flex-direction: row;
    gap: 5px;
    background-color: #ddd;
    padding: 3px 5px;
  }
  .message-row h3 {
    margin-block: auto;
  }
  .message-column {
    padding: 3px 5px;
    background-color: #eee;
  }
</style>
