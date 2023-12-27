<script lang="ts">
  import FilesGrid from "./FilesGrid.svelte";
  import FilesRows from "./FilesRows.svelte";
  import { preferences } from "../stores/preferences";
  import { workingFolder } from "../stores/workingFolder";
  import API from "../logic/api";
  import OverflowYAuto from "./OverflowYAuto.svelte";
  import type Folder from "../logic/Folder";
  import type File from "../logic/File";

  function handleFileSelect({ detail: file }: CustomEvent<File>): void {
    // TODO: Open file
  }

  function handleFolderSelect({ detail: folder }: CustomEvent<Folder>): void {
    if (folder.isEmpty || $workingFolder === null) {
      return;
    }
    const newPath = $workingFolder.clone();
    newPath.addSubFolder(folder.name);
    workingFolder.change(newPath);
  }

  function handleMore(): void {
    // TODO: Open right-click menu
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
          on:fileSelect={handleFileSelect}
          on:folderSelect={handleFolderSelect}
          on:more={handleMore}
        />
      {:else}
        <FilesRows
          {files}
          {folders}
          showOwners
          on:fileSelect={handleFileSelect}
          on:folderSelect={handleFolderSelect}
          on:more={handleMore}
        />
      {/if}
    {/await}
  {/if}
</OverflowYAuto>
