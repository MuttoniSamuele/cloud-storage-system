<script lang="ts">
  import { onMount } from "svelte";
  import FilesGrid from "./FilesGrid.svelte";

  let availableHeight: number | null = null;

  function handleWindowResize() {
    availableHeight = null;
  }

  onMount(() => {
    window.addEventListener("resize", handleWindowResize);
    return () => {
      window.removeEventListener("resize", handleWindowResize);
    };
  });
</script>

<div class="h-full overflow-y-auto" bind:offsetHeight={availableHeight}>
  <div style="height: {availableHeight !== null ? availableHeight : 0}px;">
    <FilesGrid />
  </div>
</div>
