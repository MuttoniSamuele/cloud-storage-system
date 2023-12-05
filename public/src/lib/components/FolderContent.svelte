<script lang="ts">
  import { onMount } from "svelte";
  import FilesGrid from "./FilesGrid.svelte";
  import FilesRows from "./FilesRows.svelte";
  import { preferences } from "../stores/preferences";
  import { workingFolder } from "../stores/workingFolder";
  import API from "../logic/api";

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
    {#if $workingFolder !== null}
      {#await API.getFiles($workingFolder) then files}
        {#if $preferences.filesLayout === "grid"}
          <FilesGrid {files} showOwners />
        {:else}
          <FilesRows {files} showOwners />
        {/if}
      {/await}
    {/if}
  </div>
</div>
