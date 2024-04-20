<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import File from "../logic/File";
  import Folder from "../logic/Folder";
  import API from "../logic/api";
  import { ModalState, modalState } from "../stores/modalState";
  import ContextMenu from "./ContextMenu.svelte";
  import ContextMenuDivider from "./ContextMenuDivider.svelte";
  import ContextMenuItem from "./ContextMenuItem.svelte";
  import { getCurrentPath, pathsHistory } from "../stores/pathsHistory";
  import { account } from "../stores/account";

  export let x: number;
  export let y: number;
  export let selectedFile: File | Folder;

  $: isFolder = selectedFile instanceof Folder;
  $: currentPath = getCurrentPath($pathsHistory);

  const dispatch = createEventDispatcher<{
    fileOpen: File;
    folderOpen: Folder;
  }>();
</script>

<ContextMenu {x} {y}>
  {#if !(selectedFile instanceof File && selectedFile.fileType === null)}
    <ContextMenuItem
      icon="ri-video-line"
      text="Open"
      on:click={() =>
        dispatch(isFolder ? "folderOpen" : "fileOpen", selectedFile)}
    />
    <ContextMenuDivider />
  {/if}

  <ContextMenuItem
    icon="ri-edit-line"
    text="Rename"
    on:click={() => modalState.set(ModalState.Rename)}
  />
  {#if !isFolder}
    <ContextMenuItem icon="ri-file-copy-line" text="Duplicate" />
  {/if}
  <ContextMenuItem icon="ri-folder-transfer-line" text="Move" />
  <ContextMenuDivider />

  {#if !isFolder}
    <ContextMenuItem
      icon="ri-download-line"
      text="Download"
      on:click={() => API.downloadFile(selectedFile.id)}
    />
  {/if}
  {#if currentPath?.rawPath[0].id === $account?.trashFolderId}
    <ContextMenuItem icon="ri-delete-bin-line" text="Delete" />
  {:else}
    <ContextMenuItem icon="ri-delete-bin-line" text="Move to Trash" />
  {/if}
  <ContextMenuDivider />

  <ContextMenuItem icon="ri-article-line" text="Properties" />
</ContextMenu>
