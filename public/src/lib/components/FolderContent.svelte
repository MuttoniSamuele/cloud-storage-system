<script lang="ts">
  import FilesGrid, { type ClickOutsideEvent } from "./FilesGrid.svelte";
  import FilesRows from "./FilesRows.svelte";
  import { preferences } from "../stores/preferences";
  import API from "../logic/api";
  import OverflowYAuto from "./OverflowYAuto.svelte";
  import type Folder from "../logic/Folder";
  import type File from "../logic/File";
  import { getCurrentPath, pathsHistory } from "../stores/pathsHistory";

  let contentElement: HTMLElement | null = null;
  // Set of files and folders that have been currently selected
  let selectedFiles = new Set<File | Folder>();

  $: currentPath = getCurrentPath($pathsHistory);

  function isPointInContentElement(x: number, y: number): boolean {
    if (!contentElement) {
      return false;
    }
    const { left, right, top, bottom } = contentElement.getBoundingClientRect();
    return x >= left && x <= right && y >= top && y <= bottom;
  }

  function handleClick({ detail: file }: CustomEvent<File | Folder>): void {
    // Select the file/folder
    selectedFiles.add(file);
    selectedFiles = new Set(selectedFiles);
  }

  function handleClickOutside({
    detail: { f, e },
  }: CustomEvent<ClickOutsideEvent>): void {
    // Ignore if the click happened outside of FolderContent
    if (!isPointInContentElement(e.x, e.y)) {
      return;
    }
    // Deselect the file/folder
    selectedFiles.delete(f);
    selectedFiles = new Set(selectedFiles);
  }

  function handleFileDblClick({ detail: file }: CustomEvent<File>): void {
    // TODO: Open file
  }

  function handleFolderDblClick({ detail: folder }: CustomEvent<Folder>): void {
    if (folder.isEmpty || currentPath === null) {
      return;
    }
    // Enter the folder
    const newPath = currentPath.clone();
    newPath.addSubFolder(folder.name);
    pathsHistory.push(newPath);
  }

  function handleMore(): void {
    // TODO: Open right-click menu
  }
</script>

<div class="w-full h-full" bind:this={contentElement}>
  <OverflowYAuto>
    <!-- TODO: Handle when currentPath is null and the user is logged in
  (it shows a blank page at the moment) -->
    {#if currentPath !== null}
      {#await API.getFiles(currentPath) then { files, folders }}
        {#if $preferences.filesLayout === "grid"}
          <FilesGrid
            {files}
            {folders}
            {selectedFiles}
            showOwners
            on:fileClick={handleClick}
            on:fileDblClick={handleFileDblClick}
            on:fileClickOutside={handleClickOutside}
            on:folderClick={handleClick}
            on:folderDblClick={handleFolderDblClick}
            on:folderClickOutside={handleClickOutside}
            on:more={handleMore}
          />
        {:else}
          <FilesRows
            {files}
            {folders}
            {selectedFiles}
            showOwners
            on:fileClick={handleClick}
            on:fileDblClick={handleFileDblClick}
            on:fileClickOutside={handleClickOutside}
            on:folderClick={handleClick}
            on:folderDblClick={handleFolderDblClick}
            on:folderClickOutside={handleClickOutside}
            on:more={handleMore}
          />
        {/if}
      {/await}
    {/if}
  </OverflowYAuto>
</div>
