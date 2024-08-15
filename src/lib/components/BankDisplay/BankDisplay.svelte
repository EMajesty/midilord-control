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
    <div class="bank-header">
      <div class="bank-details">
        <h1>{activeBank.name}</h1>
        <h3>{deviceConfig.active_preset}</h3>
      </div>
      <div class="preset-container">
        <p>Preset</p>
        <div class="preset-buttons">
          {#each activeBank.presets as preset, i}
            <button
              class={preset.name === deviceConfig.active_preset
                ? "selected"
                : undefined}
              on:click={() => changePreset(preset.name)}>{i + 1}</button
            >
          {/each}
        </div>
      </div>
    </div>
    <MessageList />
  {/if}
</div>

<style>
  .bank-display {
    flex: 1;
    display: flex;
    flex-direction: column;
  }
  .bank-header {
    display: flex;
    flex-direction: row;
    gap: var(--whitespace-large);
    background: linear-gradient(90deg, var(--gray-2), var(--gray-3));
    padding: var(--whitespace-large);
    border-bottom: 5px solid var(--gray-3);
  }
  .bank-details {
    padding: 10px 10px 16px 10px;
    font-size: 14px;
    height: 100px;
    width: 496px;
    color: var(--white-blue);
    letter-spacing: 3px;
    border: 5px var(--blue-5) outset;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    background: linear-gradient(-115deg, #13048548, #0729ec49, #0729ec70),
      repeating-linear-gradient(
        var(--blue-1) 0px,
        var(--blue-1) 6px,
        transparent 6px,
        transparent 42px,
        var(--blue-1) 42px
      ),
      repeating-linear-gradient(
        90deg,
        var(--blue-1) 0px,
        var(--blue-1) 6px,
        transparent 6px,
        transparent 30px,
        var(--blue-1) 30px
      ),
      repeating-linear-gradient(
        var(--blue-3) 0px,
        var(--blue-3) 1px,
        transparent 1px,
        transparent 6px,
        var(--blue-3) 6px
      ),
      repeating-linear-gradient(
        90deg,
        var(--blue-3) 0px,
        var(--blue-3) 1px,
        transparent 1px,
        transparent 6px,
        var(--blue-3) 6px
      ),
      linear-gradient(45deg, var(--blue-1), var(--blue-2));
  }
  .preset-container {
    margin-top: auto;
    display: flex;
    flex-direction: column;
    gap: 5px;
    color: #fff;
  }
  .preset-buttons {
    display: grid;
    grid-template-columns: auto auto auto auto;
  }
  .preset-container button {
    padding: 5px;
    font-size: 16px;
    background: linear-gradient(var(--gray-2), var(--gray-3));
    color: #fff;
  }
  .preset-container button:hover {
    background: linear-gradient(var(--gray-1), var(--gray-2));
  }
  .preset-container button.selected {
    background: linear-gradient(var(--gray-1), var(--gray-2));
    color: var(--blue-5);
  }
</style>
