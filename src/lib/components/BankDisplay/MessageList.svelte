<script lang="ts">
  import { get } from "svelte/store";
  import { store } from "../../../store";
  import { onDestroy } from "svelte";

  let deviceConfig = get(store).deviceConfig;

  let activeBank = deviceConfig?.banks.find(
    (bank) => bank.name === deviceConfig?.active_bank
  );

  let activePreset = activeBank?.presets.find(
    (preset) => preset.name === deviceConfig?.active_preset
  );

  let messages = activePreset?.messages;

  const unsubscribe = store.subscribe((value) => {
    deviceConfig = value.deviceConfig;
    activeBank = deviceConfig?.banks.find(
      (bank) => bank.name === deviceConfig?.active_bank
    );
    activePreset = activeBank?.presets.find(
      (preset) => preset.name === deviceConfig?.active_preset
    );
    messages = activePreset?.messages;
  });

  onDestroy(unsubscribe);
</script>

<div class="message-list">
  {#if messages}
    {#each messages as message, i}
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
  {/if}
</div>

<style>
  .message-list {
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
