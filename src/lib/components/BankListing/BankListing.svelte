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

<style>
  .bank-listing {
    background: linear-gradient(
      -215deg,
      var(--gray-1),
      var(--gray-2),
      var(--gray-2)
    );
    width: max(100px, 20%);
    padding: var(--whitespace-large);
    display: flex;
    flex-direction: column;
    border-right: 5px var(--gray-3) solid;
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
    color: #FFF;
  }
  .bank-listing button:hover,
  .bank-listing button.active:hover {
    color: var(--white-blue);
  }
  .bank-listing button.active {
    color: var(--blue-5);
  }
  .bank-listing li button:hover::before {
    content: "> ";
  }
  .bank-listing .collapse-button {
    margin-left: auto;
  }
</style>
