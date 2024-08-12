<script lang="ts">
  import type { DeviceConfig } from "$lib/utils/types";

  export let deviceConfig: DeviceConfig;
  export let uploadConfig: (config: DeviceConfig) => void;

  let collapsed = false;

  function collapse() {
    collapsed = !collapsed;
  }

  function changeBank(bankName: string) {
    const newBank = deviceConfig.banks.find((bank) => bank.name === bankName);
    if (!newBank) throw new Error("What u tryin boe");
    const newPreset = newBank.presets[0];
    uploadConfig({
      ...deviceConfig,
      active_bank: bankName,
      active_preset: newPreset.name,
    });
  }
</script>

<div class={collapsed ? "bank-listing collapsed" : "bank-listing"}>
  <button class="collapse-button" on:click={collapse}
    >{collapsed ? ">" : "<"}</button
  >
  <ul>
    {#each deviceConfig.banks as bank}
      <li>
        <button
          class={deviceConfig.active_bank === bank.name ? "active" : undefined}
          on:click={() => changeBank(bank.name)}>{bank.name}</button
        >
      </li>
    {/each}
  </ul>
</div>

<style>
  .bank-listing {
    background-color: var(--background-color-dark);
    width: max(100px, 20%);
    padding: var(--whitespace-large);
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
  }
  .bank-listing li {
    list-style: none;
  }
  .bank-listing button {
    background: none;
    border: 0;
    font-size: 15px;
    font-weight: bold;
  }
  .bank-listing button:hover, .bank-listing button.active:hover {
    color: #ccc;
  }
  .bank-listing button.active {
    color: green;
  }
  .bank-listing li button:hover::before {
    content: "> ";
  }
  .bank-listing .collapse-button {
    margin-left: auto;
  }
</style>
