<script lang="ts">
  import FilesGrid from "./FilesGrid.svelte";
  import FilesRows from "./FilesRows.svelte";
  import { preferences } from "../stores/preferences";
  import API from "../logic/api";
  import OverflowYAuto from "./OverflowYAuto.svelte";
  import type Folder from "../logic/Folder";
  import type File from "../logic/File";
  import { getCurrentPath, pathsHistory } from "../stores/pathsHistory";

  let contentElement: HTMLElement | null = null;
  let selectedFiles = new Set<File | Folder>();

  $: currentPath = getCurrentPath($pathsHistory);

  function isPointInContentElement(x: number, y: number): boolean {
    if (!contentElement) {
      return false;
    }
    const { left, right, top, bottom } = contentElement.getBoundingClientRect();
    return x >= left && x <= right && y >= top && y <= bottom;
  }

  function selectFile(file: File | Folder): void {
    selectedFiles.add(file);
    selectedFiles = new Set(selectedFiles);
  }

  function deselectFile(file: File | Folder): void {
    selectedFiles.delete(file);
    selectedFiles = new Set(selectedFiles);
  }

  function handleFileClick({ detail: file }: CustomEvent<File>): void {
    selectFile(file);
  }

  function handleFileDblClick({ detail: file }: CustomEvent<File>): void {
    // TODO: Open file
  }

  function handleFileClickOutside({
    detail: { file, e },
  }: CustomEvent<{ file: File; e: MouseEvent }>): void {
    if (isPointInContentElement(e.x, e.y)) {
      deselectFile(file);
    }
  }

  function handleFolderClick({ detail: folder }: CustomEvent<Folder>): void {
    selectFile(folder);
  }

  function handleFolderDblClick({ detail: folder }: CustomEvent<Folder>): void {
    if (folder.isEmpty || currentPath === null) {
      return;
    }
    const newPath = currentPath.clone();
    newPath.addSubFolder(folder.name);
    pathsHistory.push(newPath);
  }

  function handleFolderClickOutside({
    detail: { folder, e },
  }: CustomEvent<{ folder: Folder; e: MouseEvent }>): void {
    if (isPointInContentElement(e.x, e.y)) {
      deselectFile(folder);
    }
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
            on:fileClick={handleFileClick}
            on:fileDblClick={handleFileDblClick}
            on:fileClickOutside={handleFileClickOutside}
            on:folderClick={handleFolderClick}
            on:folderDblClick={handleFolderDblClick}
            on:folderClickOutside={handleFolderClickOutside}
            on:more={handleMore}
          />
        {:else}
          <FilesRows
            {files}
            {folders}
            {selectedFiles}
            showOwners
            on:fileClick={handleFileClick}
            on:fileDblClick={handleFileDblClick}
            on:fileClickOutside={handleFileClickOutside}
            on:folderClick={handleFolderClick}
            on:folderDblClick={handleFolderDblClick}
            on:folderClickOutside={handleFolderClickOutside}
            on:more={handleMore}
          />
        {/if}
      {/await}
    {/if}
  </OverflowYAuto>
</div>
