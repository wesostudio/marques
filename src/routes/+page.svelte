<script lang="ts">
  import { loadConfig } from "$lib/tauri-utils";
  import { onMount } from "svelte";

  import MultiStepRegister from "$lib/components/main/multi-step-register/index.svelte";

  onMount(() => {
    loadConfig();
  });
</script>

<MultiStepRegister />

<style>
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  :global(html) {
    scroll-behavior: initial;
  }
</style>
