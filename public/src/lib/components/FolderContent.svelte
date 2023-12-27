<script lang="ts">
  import FilesGrid from "./FilesGrid.svelte";
  import FilesRows from "./FilesRows.svelte";
  import { preferences } from "../stores/preferences";
  import API from "../logic/api";
  import OverflowYAuto from "./OverflowYAuto.svelte";
  import type Folder from "../logic/Folder";
  import type File from "../logic/File";
  import { getCurrentPath, pathsHistory } from "../stores/pathsHistory";

  $: currentPath = getCurrentPath($pathsHistory);

  function handleFileSelect({ detail: file }: CustomEvent<File>): void {
    // TODO: Open file
  }

  function handleFolderSelect({ detail: folder }: CustomEvent<Folder>): void {
    if (folder.isEmpty || currentPath === null) {
      return;
    }
    const newPath = currentPath.clone();
    newPath.addSubFolder(folder.name);
    pathsHistory.push(newPath);
  }

  function handleMore(): void {
    // TODO: Open right-click menu
  }
</script>

<OverflowYAuto>
  <!-- TODO: Handle when currentPath is null and the user is logged in
  (it shows a blank page at the moment) -->
  {#if currentPath !== null}
    {#await API.getFiles(currentPath) then { files, folders }}
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
