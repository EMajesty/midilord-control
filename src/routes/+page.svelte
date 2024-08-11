<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  let config = "";

  async function readConfig(){  
    config = await invoke("read_device_config");
    console.log(JSON.parse(config))
  }
</script>

<div class="container">
  <button on:click={readConfig}>Fetch</button>
  {config}
</div>

<style>
</style>
