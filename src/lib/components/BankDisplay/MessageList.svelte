<script lang="ts">
  import { get } from "svelte/store";
  import { store } from "../../../store";
  import { onDestroy } from "svelte";
  import { updateConfig } from "$lib/utils/utils";

  type DndDragEvent = DragEvent & {
    currentTarget: EventTarget & HTMLDivElement;
  };

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

  function dragStart(event: DndDragEvent, messageIndex: number) {
    const data = { messageIndex };
    event.dataTransfer?.setData("text/plain", JSON.stringify(data));
  }

  function dragOver(event: DndDragEvent) {
    event.preventDefault();
  }

  function drop(event: DndDragEvent, targetIndex: number) {
    event.preventDefault();
    const json = event.dataTransfer?.getData("text/plain");
    if (!json || !deviceConfig || !messages) return;
    const data = JSON.parse(json);
    const messageIndex = data.messageIndex;
    if (messageIndex === targetIndex) return;
    const newMessages = [...messages];
    const [movedMessage] = newMessages.splice(messageIndex, 1);
    const insertToIndex =
      messageIndex < targetIndex ? targetIndex - 1 : targetIndex;
    updateConfig({
      ...deviceConfig,
      banks: deviceConfig.banks.map((bank) => {
        if (bank.name !== deviceConfig?.active_bank) return bank;
        return {
          ...bank,
          presets: bank.presets.map((preset) => {
            if (preset.name !== deviceConfig?.active_preset) return preset;
            return {
              ...preset,
              messages: [
                ...newMessages.slice(0, insertToIndex),
                movedMessage,
                ...newMessages.slice(insertToIndex, newMessages.length),
              ],
            };
          }),
        };
      }),
    });
  }

  onDestroy(unsubscribe);
</script>

<div class="message-list">
  {#if messages}
    {#each messages as message, i (message)}
      <div
        class="message-row"
        on:dragstart={(event) => dragStart(event, i)}
        on:drop={(e) => drop(e, i)}
        draggable={true}
        on:dragover={dragOver}
      >
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
    cursor: grab;
    z-index: 0;
    position: relative;
  }
  .drag-container:global(.dragging) {
    cursor: grabbing;
    position: absolute;
    z-index: 1;
  }
  .message-row h3 {
    margin-block: auto;
  }
  .message-column {
    padding: 3px 5px;
    background-color: var(--background-color-regular);
  }
</style>
