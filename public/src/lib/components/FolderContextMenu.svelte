<script lang="ts">
  import ContextMenu from "./ContextMenu.svelte";
  import ContextMenuDivider from "./ContextMenuDivider.svelte";
  import ContextMenuItem from "./ContextMenuItem.svelte";

  // TODO: Add parameters (e.g. starred) to handle file actions
  export let x: number;
  export let y: number;
</script>

<ContextMenu {x} {y}>
  <ContextMenuItem icon="ri-video-line" text="Open" />
  <ContextMenuDivider />
  <ContextMenuItem icon="ri-edit-line" text="Rename" />
  <ContextMenuItem icon="ri-file-copy-line" text="Duplicate" />
  <ContextMenuItem icon="ri-folder-transfer-line" text="Move" />
  <ContextMenuItem icon="ri-delete-bin-line" text="Move to Trash" />
  <ContextMenuDivider />
  <ContextMenuItem icon="ri-download-line" text="Download" />
  <ContextMenuItem icon="ri-star-line" text="Star" />
  <ContextMenuDivider />
  <ContextMenuItem icon="ri-article-line" text="Properties" />
</ContextMenu>
