<script lang="ts">
  import FilesGrid, { type ClickPositionEvent } from "./FilesGrid.svelte";
  import FilesRows from "./FilesRows.svelte";
  import FolderContextMenu from "./FolderContextMenu.svelte";
  import { preferences } from "../stores/preferences";
  import API from "../logic/api";
  import OverflowYAuto from "./OverflowYAuto.svelte";
  import type Folder from "../logic/Folder";
  import type File from "../logic/File";
  import { getCurrentPath, pathsHistory } from "../stores/pathsHistory";

  let contentElement: HTMLElement | null = null;
  // Set of files and folders that have been currently selected
  let selectedFiles = new Set<File | Folder>();

  // Context menu
  let isContextMenuOpen = false;
  let contextMenuX = 0;
  let contextMenuY = 0;

  $: currentPath = getCurrentPath($pathsHistory);

  function isPointInContentElement(x: number, y: number): boolean {
    if (!contentElement) {
      return false;
    }
    const { left, right, top, bottom } = contentElement.getBoundingClientRect();
    return x >= left && x <= right && y >= top && y <= bottom;
  }

  function selectFileFolder(f: File | Folder): void {
    selectedFiles.add(f);
    selectedFiles = new Set(selectedFiles);
  }

  function deselectFileFolder(f: File | Folder): void {
    selectedFiles.delete(f);
    selectedFiles = new Set(selectedFiles);
  }

  function handleClick({ detail: f }: CustomEvent<File | Folder>): void {
    isContextMenuOpen = false;
    selectFileFolder(f);
  }

  function handleClickOutside({
    detail: { f, e },
  }: CustomEvent<ClickPositionEvent>): void {
    isContextMenuOpen = false;
    // Ignore if the click happened outside of FolderContent
    if (!isPointInContentElement(e.x, e.y)) {
      return;
    }
    deselectFileFolder(f);
  }

  function handleFileDblClick({ detail: file }: CustomEvent<File>): void {
    // TODO: Open file
  }

  function handleFolderDblClick({ detail: folder }: CustomEvent<Folder>): void {
    if (currentPath === null) {
      return;
    }
    // Enter the folder
    const newPath = currentPath.clone();
    newPath.addSubFolder(folder.name);
    pathsHistory.push(newPath);
  }

  function handleContextMenu({
    detail: { f, e },
  }: CustomEvent<ClickPositionEvent>): void {
    isContextMenuOpen = true;
    contextMenuX = e.x;
    contextMenuY = e.y;
    for (const file of selectedFiles) {
      deselectFileFolder(file);
    }
    selectFileFolder(f);
  }

  function handleMore(): void {
    // TODO: Open right-click menu
  }
</script>

<div class="w-full h-full p-4" bind:this={contentElement}>
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
            on:fileContextMenu={handleContextMenu}
            on:folderClick={handleClick}
            on:folderDblClick={handleFolderDblClick}
            on:folderClickOutside={handleClickOutside}
            on:folderContextMenu={handleContextMenu}
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
            on:fileContextMenu={handleContextMenu}
            on:folderClick={handleClick}
            on:folderDblClick={handleFolderDblClick}
            on:folderClickOutside={handleClickOutside}
            on:folderContextMenu={handleContextMenu}
            on:more={handleMore}
          />
        {/if}
      {/await}
    {/if}
  </OverflowYAuto>
</div>
<!-- TODO: Fix context menu position on window resize -->
{#if isContextMenuOpen}
  <FolderContextMenu x={contextMenuX} y={contextMenuY} />
{/if}
