<script lang="ts">
  import { onMount } from "svelte";
  import FilesGrid from "./FilesGrid.svelte";
  import FilesRows from "./FilesRows.svelte";
  import { preferences } from "../stores/preferences";

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

<!-- TODO: add file icons for images and text files -->

<div class="h-full overflow-y-auto" bind:offsetHeight={availableHeight}>
  <div style="height: {availableHeight !== null ? availableHeight : 0}px;">
    {#if $preferences.filesLayout === "grid"}
      <FilesGrid />
    {:else}
      <FilesRows showOwners />
    {/if}
  </div>
</div>
