<script lang="ts">
  import ContextMenu from "./ContextMenu.svelte";
  import ContextMenuDivider from "./ContextMenuDivider.svelte";
  import ContextMenuItem from "./ContextMenuItem.svelte";

  export let x: number;
  export let y: number;
</script>

<ContextMenu {x} {y}>
  <ContextMenuItem icon="ri-video-line" text="Open" />
  <ContextMenuDivider />
  <ContextMenuItem icon="ri-edit-line" text="Rename" />
  <ContextMenuItem icon="ri-file-copy-line" text="Copy" />
  <ContextMenuItem icon="ri-scissors-line" text="Cut" />
  <ContextMenuItem icon="ri-delete-bin-line" text="Delete" />
  <ContextMenuDivider />
  <ContextMenuItem icon="ri-download-line" text="Download" />
  <ContextMenuItem icon="ri-share-line" text="Share" />
  <ContextMenuDivider />
  <ContextMenuItem icon="ri-article-line" text="Properties" />
</ContextMenu>
