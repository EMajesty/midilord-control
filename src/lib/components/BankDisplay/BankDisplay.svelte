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
      </div>
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
    </div>
    <MessageList />
  {/if}
</div>

<style>
  .bank-display {
    flex: 1;
    padding: var(--whitespace-large);
    gap: var(--whitespace-large);
    display: flex;
    flex-direction: column;
  }
  .bank-header {
    display: flex;
    flex-direction: row;
    gap: var(--whitespace-large);
  }
  .bank-details {
    padding: 10px;
    font-size: 14px;
    height: 100px;
    width: 496px;
    color: var(--white-blue);
    letter-spacing: 3px;
    border: 5px var(--blue-5) outset;
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
