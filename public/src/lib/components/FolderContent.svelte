<script lang="ts">
  import FilesGrid from "./FilesGrid.svelte";
  import FilesRows from "./FilesRows.svelte";
  import { preferences } from "../stores/preferences";
  import { workingFolder } from "../stores/workingFolder";
  import API from "../logic/api";
  import OverflowYAuto from "./OverflowYAuto.svelte";
</script>

<OverflowYAuto>
  {#if $workingFolder !== null}
    {#await API.getFiles($workingFolder) then { files, folders }}
      {#if $preferences.filesLayout === "grid"}
        <FilesGrid {files} {folders} showOwners />
      {:else}
        <FilesRows {files} {folders} showOwners />
      {/if}
    {/await}
  {/if}
</OverflowYAuto>
