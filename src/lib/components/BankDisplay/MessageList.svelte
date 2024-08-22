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

  function dragOverMiddlePoint(event: DndDragEvent) {
    const { pageY, currentTarget } = event;
    const { y, height } = currentTarget.getBoundingClientRect();
    const middlePoint = y + height / 2;
    return middlePoint > pageY;
  }

  function dragOver(event: DndDragEvent) {
    event.preventDefault();
    const { currentTarget } = event;
    currentTarget.classList.add("targeted");
    if (dragOverMiddlePoint(event)) {
      currentTarget.classList.remove("targeted-above");
      currentTarget.classList.add("targeted-below");
    } else {
      currentTarget.classList.remove("targeted-below");
      currentTarget.classList.add("targeted-above");
    }
  }

  function clearDragClasses(currentTarget: EventTarget & HTMLDivElement) {
    currentTarget.classList.remove("targeted");
    currentTarget.classList.remove("targeted-above");
    currentTarget.classList.remove("targeted-below");
  }

  function dragLeave(event: DndDragEvent) {
    event.preventDefault();
    clearDragClasses(event.currentTarget);
  }

  // TODO cleanup and test this more
  function drop(event: DndDragEvent, targetIndex: number) {
    event.preventDefault();
    clearDragClasses(event.currentTarget);
    const json = event.dataTransfer?.getData("text/plain");
    if (!json || !deviceConfig || !messages) return;
    const data = JSON.parse(json);
    const messageIndex = data.messageIndex;
    if (messageIndex === targetIndex) return;
    const newMessages = [...messages];
    const [movedMessage] = newMessages.splice(messageIndex, 1);
    let insertToIndex = 0;
    const insertToBefore = dragOverMiddlePoint(event);
    if (insertToBefore) {
      insertToIndex =
        targetIndex === 0
          ? 0
          : messageIndex < targetIndex
            ? targetIndex - 1
            : targetIndex;
    } else {
      insertToIndex =
        targetIndex === newMessages.length
          ? newMessages.length + 1
          : messageIndex < targetIndex
            ? targetIndex
            : targetIndex + 1;
    }
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
        class={"message-row"}
        on:dragstart={(event) => dragStart(event, i)}
        on:drop={(e) => drop(e, i)}
        draggable={true}
        on:dragover={dragOver}
        on:dragleave={dragLeave}
      >
        <div class="message-header">
          <h3>Msg {i + 1}</h3>
        </div>
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
    background-color: var(--background-color-regular);
    padding: var(--whitespace-large);
    gap: 9px;
    min-width: var(--content-min-width);
    min-height: 0;
    overflow-y: overlay;
  }
  .message-row {
    display: flex;
    flex-direction: row;
    gap: var(--whitespace-large);
    background-color: var(--blue-4);
    cursor: grab;
    z-index: 0;
    position: relative;
    border: 5px outset var(--blue-4);
    position: relative;
  }
  .message-row:global(.targeted) {
    border-style: inset;
  }
  .message-row:global(.targeted)::before {
    content: "";
    width: calc(100% + 10px);
    height: 5px;
    background: linear-gradient(90deg, var(--white-blue), var(--blue-5));
    position: absolute;
    left: -5px;
  }
  .message-row:global(.targeted-below)::before {
    top: -12px;
  }
  .message-row:global(.targeted-above)::before {
    bottom: -12px;
  }
  .message-header {
    display: flex;
    background: linear-gradient(90deg, var(--blue-5), var(--blue-4));
    color: var(--white-blue);
  }
  .message-header h3 {
    margin-block: auto;
  }
  .message-header,
  .message-column {
    padding: var(--whitespace-large);
    color: #fff;
    pointer-events: none;
  }
  .message-column {
    background-color: var(--background-color-regular);
    font-size: 12px;
    display: flex;
    flex-direction: column;
    gap: var(--whitespace-medium);
  }
</style>
