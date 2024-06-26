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
  import { fileMove } from "../stores/fileMove";
  import { fileChange } from "../stores/fileChange";
  import App from "../../App.svelte";

  export let x: number;
  export let y: number;
  export let selectedFile: File | Folder;

  $: isFolder = selectedFile instanceof Folder;
  $: currentPath = getCurrentPath($pathsHistory);
  $: isTrash =
    currentPath !== null &&
    $account !== null &&
    currentPath.rawPath[0].id === $account.trashFolderId;

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

  {#if !isTrash}
    <ContextMenuItem
      icon="ri-edit-line"
      text="Rename"
      on:click={() => modalState.set(ModalState.Rename)}
    />
    {#if !isFolder}
      <ContextMenuItem
        icon="ri-file-copy-line"
        text="Duplicate"
        on:click={async () => {
          await API.duplicateFile(selectedFile.id);
          fileChange.setFile(selectedFile.name);
          pathsHistory.refresh();
        }}
      />
    {/if}
    <ContextMenuItem
      icon="ri-folder-transfer-line"
      text="Move"
      on:click={() => fileMove.set(selectedFile)}
    />
    <ContextMenuDivider />
  {/if}

  {#if !isFolder}
    <ContextMenuItem
      icon="ri-download-line"
      text="Download"
      on:click={async () => await API.downloadFile(selectedFile.id)}
    />
  {/if}
  {#if isTrash}
    <ContextMenuItem
      icon="ri-arrow-go-back-line"
      text="Restore"
      on:click={async () => {
        await API.moveFile(
          selectedFile.id,
          $account?.personalFolderId ?? 0,
          isFolder,
        );
        fileChange.setFile(selectedFile.name);
        pathsHistory.refresh();
      }}
    />
    <ContextMenuItem
      icon="ri-delete-bin-line"
      text="Delete"
      on:click={async () => {
        await API.deleteFile(selectedFile.id, isFolder);
        fileChange.setFile(selectedFile.name);
        pathsHistory.refresh();
      }}
    />
  {:else}
    <ContextMenuItem
      icon="ri-delete-bin-line"
      text="Move to Trash"
      on:click={async () => {
        if (selectedFile.id === $fileMove?.id) {
          fileMove.cancel();
        }
        await API.moveFile(
          selectedFile.id,
          $account?.trashFolderId ?? 0,
          isFolder,
        );
        pathsHistory.refresh();
      }}
    />
  {/if}
  <ContextMenuDivider />

  <ContextMenuItem
    icon="ri-article-line"
    text="Properties"
    on:click={() => modalState.set(ModalState.Properties)}
  />
</ContextMenu>
