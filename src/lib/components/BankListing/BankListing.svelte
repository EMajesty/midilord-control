<script lang="ts">
  import { updateConfig } from "$lib/utils/utils";
  import { get } from "svelte/store";
  import { store } from "../../../store";
  import { onDestroy } from "svelte";

  let searchText = "";
  let collapsed = false;
  let deviceConfig = get(store).deviceConfig;
  const unsubscribe = store.subscribe((value) => {
    deviceConfig = value.deviceConfig;
  });
  $: banks = searchText
    ? deviceConfig.banks.filter((bank) => bank.name.includes(searchText))
    : deviceConfig.banks;

  function collapse() {
    collapsed = !collapsed;
  }

  function changeBank(bankName: string) {
    const newBank = deviceConfig?.banks.find((bank) => bank.name === bankName);
    if (!deviceConfig || !newBank) return;
    const newPreset = newBank.presets[0];
    updateConfig({
      ...deviceConfig,
      active_bank: bankName,
      active_preset: newPreset.name,
    });
  }

  function onSearch(
    e: Event & {
      currentTarget: EventTarget & HTMLInputElement;
    }
  ) {
    searchText = e.currentTarget.value;
  }

  onDestroy(unsubscribe);
</script>

<div class={collapsed ? "bank-listing collapsed" : "bank-listing"}>
  <div class="list-functions">
    <input class="search-input" placeholder="Search..." on:input={onSearch} />
    <button class="collapse-button" on:click={collapse}
      >{collapsed ? ">" : "<"}</button
    >
  </div>
  <div class="list-wrapper">
    {#if banks.length}
      <ul>
        {#each banks as bank}
          <li>
            <button
              class={deviceConfig.active_bank === bank.name
                ? "active"
                : undefined}
              on:click={() => changeBank(bank.name)}>{bank.name}</button
            >
          </li>
        {/each}
      </ul>
    {:else}
      <p>No results</p>
    {/if}
  </div>
</div>

<style>
  .bank-listing {
    z-index: 1;
    border-radius: 0 0 16px 0;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.26);
    border-right: 1px solid rgba(18, 19, 17, 0.26);
    width: clamp(200px, 250px, 50vw);
    background: linear-gradient(-45deg, #121311f1, rgba(39, 42, 35, 0.849));
    max-height: 100vh;
    display: flex;
    flex-direction: column;
  }

  .list-functions {
    padding: var(--whitespace-large);
    padding-right: 6px;
    display: flex;
    gap: 5px;
  }
  .list-wrapper {
    background: rgba(45, 48, 43, 0.431);
    border-radius: 0 0 16px 0;
    backdrop-filter: blur(7.5px);
    -webkit-backdrop-filter: blur(7.5px);
    padding: var(--whitespace-large);
    flex: 1;
    display: flex;
    flex-direction: column;
    max-height: 100%;
    overflow-y: auto;
  }
  .list-wrapper p {
    text-align: center;
    color: #fff;
  }
  .bank-listing.collapsed {
    width: 45px;
  }
  .bank-listing.collapsed *:not(.list-functions):not(.collapse-button) {
    display: none;
  }
  .bank-listing ul {
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: var(--whitespace-large);
  }
  .bank-listing li {
    list-style: none;
  }
  .bank-listing button {
    background: none;
    border: 0;
    font-size: 15px;
    font-weight: bold;
    color: var(--blue-4);
  }
  .bank-listing button:hover,
  .bank-listing button.active:hover {
    color: var(--white-blue);
  }
  .bank-listing button.active {
    color: var(--white-blue);
  }
  .bank-listing li button:hover::before {
    content: "> ";
  }
</style>
