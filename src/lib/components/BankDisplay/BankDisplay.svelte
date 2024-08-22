<script lang="ts">
  import { updateConfig } from "$lib/utils/utils";
  import { onDestroy } from "svelte";
  import MessageList from "./MessageList.svelte";
  import { store } from "../../../store";
  import { get } from "svelte/store";

  let deviceConfig = get(store).deviceConfig;
  let activeBank = deviceConfig.banks.find(
    (bank) => bank.name === deviceConfig?.active_bank
  );
  const unsubscribe = store.subscribe((value) => {
    deviceConfig = value.deviceConfig;
    activeBank = deviceConfig.banks.find(
      (bank) => bank.name === deviceConfig?.active_bank
    );
  });

  function changePreset(presetName: string) {
    if (!deviceConfig) return;
    updateConfig({ ...deviceConfig, active_preset: presetName });
  }

  onDestroy(unsubscribe);

  const mapRow = (text: string) => {
    const textMaxLength = 12;
    const chars = text.split("").slice(0, textMaxLength);
    for (let i = chars.length; i < textMaxLength; i++) {
      chars.push("");
    }
    return chars;
  };

  $: bankNameChars = mapRow(activeBank?.name ?? "");
  $: presetNameChars = mapRow(deviceConfig.active_preset ?? "");
</script>

<div class="bank-display">
  {#if activeBank}
    <div class="bank-header">
      <div class="bank-details">
        <div class="title-row">
          {#each bankNameChars as char}
            <span class="char-wrapper"><span>{char}</span></span>
          {/each}
        </div>
        <div class="title-row">
          {#each presetNameChars as char}
            <span class="char-wrapper"><span>{char}</span></span>
          {/each}
        </div>
        <div class="glass-overlay" />
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
    overflow-x: auto;
    min-width: 0;
    max-height: 100vh;
  }
  .bank-header {
    display: flex;
    flex-direction: row;
    gap: var(--whitespace-large);
    background: linear-gradient(-45deg, #121311f1, rgba(39, 42, 35, 0.849)),
      repeating-linear-gradient(
        -35deg,
        var(--gray-1) 0px,
        var(--gray-2) 3px,
        var(--gray-3) 6px
      );
    padding: var(--whitespace-large);
    border-top: 5px solid var(--gray-3);
    border-left: 5px solid var(--gray-3);
    border-right: 5px solid var(--gray-2);
    border-bottom: 5px solid var(--gray-2);
    box-shadow: 0 0 20px rgba(0, 0, 0, 0.5);
    min-width: var(--content-min-width);
  }
  .bank-details {
    padding: 5px;
    font-size: 14px;
    height: 100px;
    width: 369px;
    min-width: 369px;
    color: var(--white-blue);
    letter-spacing: 3px;
    border: 5px var(--blue-5) outset;
    display: flex;
    gap: 6px;
    flex-direction: column;
    justify-content: space-between;
    background: linear-gradient(45deg, var(--blue-1), var(--blue-2));
    position: relative;
  }
  .glass-overlay {
    width: 100%;
    height: 100%;
    background: linear-gradient(35deg,#5a77c733, #0729ec2f, #8199da1c, #2b54c41f);
    position: absolute;
    left: 0;
    top: 0;
  }
  .title-row {
    flex: 1;
    display: flex;
    flex-direction: row;
    gap: 5px;
  }
  .title-row:first-child .char-wrapper {
    font-size: 22px;
    font-weight: bold;
  }
  .char-wrapper {
    flex: 1;
    font-size: 18px;
    display: flex;
    padding: 2px;
    background: repeating-linear-gradient(
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
      );
  }
  .char-wrapper span {
    margin: auto;
    letter-spacing: 0;
    line-height: 0;
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
