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
  import type Path from "../logic/Path";
  import { selectedFiles } from "../stores/selectedFile";
  import { ModalState, modalState } from "../stores/modalState";

  let contentElement: HTMLElement | null = null;
  // Set of files and folders that have been currently selected
  // let selectedFiles = new Set<File | Folder>();

  // Context menu
  let isContextMenuOpen = false;
  let contextMenuX = 0;
  let contextMenuY = 0;
  // A flag used to prevent the clickOutside event from overriding the
  // more button click event
  let preventNextClose = false;

  let currentPath: Path | null = null;
  // This block runs every time currentPath changes, so it closes the
  // context menu when the displayed files change
  $: {
    currentPath = getCurrentPath($pathsHistory);
    isContextMenuOpen = false;
  }

  function isPointInContentElement(x: number, y: number): boolean {
    if (!contentElement) {
      return false;
    }
    const { left, right, top, bottom } = contentElement.getBoundingClientRect();
    return x >= left && x <= right && y >= top && y <= bottom;
  }

  function selectFileFolder(f: File | Folder): void {
    selectedFiles.add(f);
  }

  function deselectFileFolder(f: File | Folder): void {
    selectedFiles.delete(f);
  }

  function handleClick({ detail: f }: CustomEvent<File | Folder>): void {
    isContextMenuOpen = false;
    selectFileFolder(f);
  }

  function handleClickOutside({
    detail: { f, e },
  }: CustomEvent<ClickPositionEvent>): void {
    // Hacky way to prevent the clickOutside event from overriding the
    // more button click event
    if (selectedFiles.has(f)) {
      if (preventNextClose) {
        preventNextClose = false;
        // Exit here so the file doesn't get deselected
        return;
      } else {
        isContextMenuOpen = false;
      }
    }
    // Ignore if the click happened outside of FolderContent
    if (!isPointInContentElement(e.x, e.y)) {
      return;
    }
    deselectFileFolder(f);
  }

  function handleFileDblClick({ detail: file }: CustomEvent<File>): void {
    switch (file.fileType) {
      case "Text": {
        modalState.set(ModalState.TextFile);
        break;
      }
      case "Image": {
        modalState.set(ModalState.ImageFile);
        break;
      }
      default: {
        modalState.set(ModalState.UnsupportedFile);
      }
    }
  }

  function handleFolderDblClick({ detail: folder }: CustomEvent<Folder>): void {
    if (currentPath === null) {
      return;
    }
    // Enter the folder
    const newPath = currentPath.clone();
    newPath.addSubFolder({ id: folder.id, name: folder.name });
    pathsHistory.push(newPath);
  }

  function handleRightClick({
    detail: { f, e },
  }: CustomEvent<ClickPositionEvent>): void {
    isContextMenuOpen = true;
    contextMenuX = e.x;
    contextMenuY = e.y;
    for (const file of $selectedFiles) {
      deselectFileFolder(file);
    }
    selectFileFolder(f);
  }

  function handleMore(e: CustomEvent<ClickPositionEvent>): void {
    handleRightClick(e);
    preventNextClose = true;
  }
</script>

<svelte:window on:resize={() => (isContextMenuOpen = false)} />

<div class="w-full h-full p-4" bind:this={contentElement}>
  <OverflowYAuto on:scroll={() => (isContextMenuOpen = false)}>
    {#if currentPath !== null}
      {#await API.view(currentPath.lastId) then { files, folders }}
        {#if $preferences.filesLayout === "grid"}
          <FilesGrid
            {files}
            {folders}
            selectedFiles={$selectedFiles}
            on:fileClick={handleClick}
            on:fileDblClick={handleFileDblClick}
            on:fileClickOutside={handleClickOutside}
            on:fileRightClick={handleRightClick}
            on:fileMore={handleMore}
            on:folderClick={handleClick}
            on:folderDblClick={handleFolderDblClick}
            on:folderClickOutside={handleClickOutside}
            on:folderRightClick={handleRightClick}
            on:folderMore={handleMore}
          />
        {:else}
          <FilesRows
            {files}
            {folders}
            selectedFiles={$selectedFiles}
            on:fileClick={handleClick}
            on:fileDblClick={handleFileDblClick}
            on:fileClickOutside={handleClickOutside}
            on:fileRightClick={handleRightClick}
            on:fileMore={handleMore}
            on:folderClick={handleClick}
            on:folderDblClick={handleFolderDblClick}
            on:folderClickOutside={handleClickOutside}
            on:folderRightClick={handleRightClick}
            on:folderMore={handleMore}
          />
        {/if}
      {/await}
    {/if}
  </OverflowYAuto>
</div>
{#if isContextMenuOpen}
  <FolderContextMenu
    x={contextMenuX}
    y={contextMenuY}
    selectedFile={Array.from($selectedFiles)[0]}
    on:fileOpen={handleFileDblClick}
    on:folderOpen={handleFolderDblClick}
  />
{/if}
