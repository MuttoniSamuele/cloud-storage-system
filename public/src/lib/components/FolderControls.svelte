<script lang="ts">
  import { account } from "../stores/account";
  import { fileMove } from "../stores/fileMove";
  import { getCurrentPath, pathsHistory } from "../stores/pathsHistory";
  import Breadcrumb from "./Breadcrumb.svelte";
  import EmptyTrashButton from "./EmptyTrashButton.svelte";
  import FilesLayoutToggle from "./FilesLayoutToggle.svelte";
  import MoveControls from "./MoveControls.svelte";
  // import IconButton from "./IconButton.svelte";
  import Navigation from "./Navigation.svelte";
  import NewFolderButton from "./NewFolderButton.svelte";
  import RefreshButton from "./RefreshButton.svelte";
  import UploadButton from "./UploadButton.svelte";

  $: currentPath = getCurrentPath($pathsHistory);
  $: isTrash =
    currentPath !== null &&
    $account !== null &&
    currentPath.rawPath[0].id === $account.trashFolderId;
</script>

<div class="flex m-4">
  <div class="flex items-center whitespace-nowrap flex-1 mr-2">
    <Navigation />
    <RefreshButton />
    <Breadcrumb />
  </div>
  <div class="flex items-center whitespace-nowrap">
    <FilesLayoutToggle />
    <!-- TODO: Implement client-side filtering (by extension) -->
    <!-- <IconButton icon="ri-filter-line" margin /> -->
    {#if isTrash}
      <EmptyTrashButton />
    {:else if $fileMove !== null}
      <MoveControls file={$fileMove} />
    {:else}
      <UploadButton />
      <NewFolderButton />
    {/if}
  </div>
</div>
