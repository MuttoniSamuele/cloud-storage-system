<script lang="ts">
  import type File from "../logic/File";
  import Folder from "../logic/Folder";
  import API from "../logic/api";
  import { ModalState, modalState } from "../stores/modalState";
  import ContextMenu from "./ContextMenu.svelte";
  import ContextMenuDivider from "./ContextMenuDivider.svelte";
  import ContextMenuItem from "./ContextMenuItem.svelte";

  export let x: number;
  export let y: number;
  export let selectedFile: File | Folder;

  $: isFolder = selectedFile instanceof Folder;
</script>

<ContextMenu {x} {y}>
  <ContextMenuItem icon="ri-video-line" text="Open" />
  <ContextMenuDivider />
  <ContextMenuItem
    icon="ri-edit-line"
    text="Rename"
    on:click={() => modalState.set(ModalState.Rename)}
  />
  {#if !isFolder}
    <ContextMenuItem icon="ri-file-copy-line" text="Duplicate" />
  {/if}
  <ContextMenuItem icon="ri-folder-transfer-line" text="Move" />
  <ContextMenuItem icon="ri-delete-bin-line" text="Move to Trash" />
  <ContextMenuDivider />
  {#if !isFolder}
    <ContextMenuItem
      icon="ri-download-line"
      text="Download"
      on:click={() => API.downloadFile(selectedFile.id)}
    />
  {/if}
  <ContextMenuItem icon="ri-star-line" text="Star" />
  <ContextMenuDivider />
  <ContextMenuItem icon="ri-article-line" text="Properties" />
</ContextMenu>
