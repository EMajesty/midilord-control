<script lang="ts">
  import { get } from "svelte/store";
  import { store } from "../../../store";
  import { onDestroy } from "svelte";
  import { editMessage, moveMessage } from "$lib/utils/utils";
  import { MessageType } from "$lib/utils/types";

  type DndDragEvent = DragEvent & {
    currentTarget: EventTarget & HTMLDivElement;
  };

  let deviceConfig = get(store).config;
  let messages = get(store).messages;

  const unsubscribe = store.subscribe((value) => {
    deviceConfig = value.config;
    messages = value.messages;
  });

  $: displayedMessages = messages.filter(
    ({ message_type }) => message_type !== MessageType.EMPTY,
  );

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

  function drop(event: DndDragEvent, targetIndex: number) {
    event.preventDefault();
    clearDragClasses(event.currentTarget);
    const json = event.dataTransfer?.getData("text/plain");
    if (!json || !deviceConfig || !messages) return;
    const data = JSON.parse(json);
    const messageIndex = data.messageIndex;
    let insertToIndex = 0;
    const insertToBefore = dragOverMiddlePoint(event);
    if (insertToBefore) {
      insertToIndex = targetIndex;
    } else {
      insertToIndex = targetIndex + 1;
    }
    if (messageIndex === insertToIndex) return;
    moveMessage(messageIndex, insertToIndex);
  }

  function addMessage() {
    const messageIndex = messages.findIndex(
      ({ message_type }) => message_type === MessageType.EMPTY,
    );
    if (messageIndex < 0) return;
    editMessage(messageIndex, {
      ...messages[messageIndex],
      message_type: MessageType.INTERNAL,
    });
  }

  onDestroy(unsubscribe);
</script>

<div class="message-list">
  {#each displayedMessages as message, i (message)}
    <div
      class={"message-row"}
      on:dragstart={(event) => dragStart(event, i)}
      on:drop={(e) => drop(e, i)}
      draggable={true}
      on:dragover={dragOver}
      on:dragleave={dragLeave}>
      <div class="message-header">
        <h3>Msg {i + 1}</h3>
      </div>
      <div class="message-column">
        <b>Type</b>
        <p>{message.message_type}</p>
      </div>
      <div class="message-column">
        <b>Channel</b>
        <p>{message.message_channel}</p>
      </div>
      <div class="message-column">
        <b>Number</b>
        <p>{message.message_number}</p>
      </div>
      <div class="message-column">
        <b>Value</b>
        <p>{message.message_value}</p>
      </div>
    </div>
  {/each}
  <button
    class="add-message-button"
    on:click={addMessage}
    disabled={displayedMessages.length === messages.length}>Add message</button>
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
    background-color: var(--blue-4);
    cursor: grab;
    z-index: 0;
    position: relative;
    border: 5px outset var(--blue-4);
    position: relative;
  }
  .message-row > div:nth-child(2) {
    padding-inline: 24px;
    flex: 1;
  }
  @property --gradient-angle {
    syntax: "<angle>";
    initial-value: 0deg;
    inherits: false;
  }
  @keyframes rotation {
    0% {
      --gradient-angle: 0deg;
    }
    100% {
      --gradient-angle: 360deg;
    }
  }
  .message-row:hover,
  .message-row:global(.targeted) {
    border: 0;
    margin: 5px;
  }

  .message-row:hover::before,
  .message-row:hover::after,
  .message-row:global(.targeted)::before,
  .message-row:global(.targeted)::after {
    content: "";
    position: absolute;
    inset: -5px;
    border-radius: inherit;
    z-index: -1;
    animation: rotation 10s linear infinite;
  }
  .message-row:hover::before,
  .message-row:hover::after {
    background: conic-gradient(
      from var(--gradient-angle),
      var(--blue-4),
      var(--blue-5),
      var(--blue-4)
    );
  }

  .message-row:global(.targeted)::before,
  .message-row:global(.targeted)::after {
    background: conic-gradient(
      from var(--gradient-angle),
      #fff,
      var(--white-blue),
      #fff
    );
  }

  .message-row:hover::after,
  .message-row:global(.targeted)::after {
    filter: blur(5px);
  }
  .message-row:global(.targeted) .message-header::before {
    content: "";
    width: calc(100% + 10px);
    height: 5px;
    background: linear-gradient(90deg, var(--white-blue), var(--blue-5));
    position: absolute;
    left: -5px;
  }
  .message-row:global(.targeted-below) .message-header::before {
    top: -12px;
  }
  .message-row:global(.targeted-above) .message-header::before {
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
    background-color: var(--blue-4);
    font-size: 12px;
    display: flex;
    flex-direction: column;
    gap: var(--whitespace-medium);
  }
  .add-message-button {
    position: fixed;
    bottom: 10px;
    right: 150px;
  }
</style>
