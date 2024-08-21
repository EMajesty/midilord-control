<script lang="ts">
  import { updateConfig } from "$lib/utils/utils";
  import { get } from "svelte/store";
  import { store } from "../../../store";
  import { onDestroy } from "svelte";

  let collapsed = false;
  let deviceConfig = get(store).deviceConfig;
  const unsubscribe = store.subscribe((value) => {
    deviceConfig = value.deviceConfig;
  });

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

  onDestroy(unsubscribe);
</script>

<div class={collapsed ? "bank-listing collapsed" : "bank-listing"}>
  <div class="list-wrapper">
  {#if deviceConfig}
    <button class="collapse-button" on:click={collapse}
      >{collapsed ? ">" : "<"}</button
    >
    <ul>
      {#each deviceConfig.banks as bank}
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
  {/if}
</div>
</div>

<style>
  .bank-listing {
    z-index: 1;
    border-radius: 0 0 16px 0;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.26);
    border-right: 1px solid rgba(18, 19, 17, 0.26);
    width: max(100px, 20%);
    display: flex;
    background: linear-gradient(-45deg, #121311f1, rgba(39, 42, 35, 0.849));
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

  }
  .bank-listing.collapsed {
    width: initial;
  }
  .bank-listing.collapsed ul {
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
  .bank-listing .collapse-button {
    margin-left: auto;
  }
</style>
