<script lang="ts">
  import { get } from "svelte/store";
  import { flip } from "svelte/animate";
  import { store } from "../../../store";
  import { onDestroy } from "svelte";
  import { updateConfig } from "$lib/utils/utils";
  import type { Message } from "$lib/utils/types";

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

  function dragStart(event: any, messageIndex: number) {
    const data = { messageIndex };
    event.dataTransfer.setData("text/plain", JSON.stringify(data));
  }

  function drop(event: any, targetIndex: number) {
    console.log(event, targetIndex);
    if (!deviceConfig || !messages) return;
    event.preventDefault();
    const json = event.dataTransfer.getData("text/plain");
    const data = JSON.parse(json);
    const messageIndex = data.messageIndex;
    const movedMessage = messages[messageIndex];
    const newMessages: Message[] = [];
    for (let i = 0; i < targetIndex; i++) {
      newMessages.push(messages[i]);
    }
    newMessages.push(movedMessage);
    for (let i = targetIndex; i < messageIndex; i++) {
      newMessages.push(messages[i]);
    }
    for (let i = messageIndex + 1; i < messages.length - 1; i++) {
      newMessages.push(messages[i]);
    }
    messages = newMessages;
  }

  onDestroy(unsubscribe);
</script>

<div class="message-list">
  {#if messages}
    {#each messages as message, i (message)}
      <div
        class="message-row"
        animate:flip
        on:dragstart={(event) => dragStart(event, i)}
        on:drop={(e) => drop(e, i)}
        draggable={true}
        on:dragover={() => false}
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
