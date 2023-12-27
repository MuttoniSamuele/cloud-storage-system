<script lang="ts">
  import FilesGrid from "./FilesGrid.svelte";
  import FilesRows from "./FilesRows.svelte";
  import { preferences } from "../stores/preferences";
  import { workingFolder } from "../stores/workingFolder";
  import API from "../logic/api";
  import OverflowYAuto from "./OverflowYAuto.svelte";
  import type FileBase from "../logic/FileBase";

  function handleSelect({ detail: file }: CustomEvent<FileBase>): void {
    console.log(file.name);
  }

  function handleMore(): void {
    console.log("More");
  }
</script>

<OverflowYAuto>
  {#if $workingFolder !== null}
    {#await API.getFiles($workingFolder) then { files, folders }}
      {#if $preferences.filesLayout === "grid"}
        <FilesGrid
          {files}
          {folders}
          showOwners
          on:select={handleSelect}
          on:more={handleMore}
        />
      {:else}
        <FilesRows
          {files}
          {folders}
          showOwners
          on:select={handleSelect}
          on:more={handleMore}
        />
      {/if}
    {/await}
  {/if}
</OverflowYAuto>
