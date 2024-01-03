<script lang="ts">
  import Breadcrumb from "./Breadcrumb.svelte";
  import FilesLayoutToggle from "./FilesLayoutToggle.svelte";
  import IconButton from "./IconButton.svelte";
  import Navigation from "./Navigation.svelte";
  import RefreshButton from "./RefreshButton.svelte";
  import TextButton from "./TextButton.svelte";
</script>

<div class="flex m-4">
  <div class="flex items-center whitespace-nowrap flex-1 mr-2">
    <Navigation />
    <RefreshButton />
    <Breadcrumb />
  </div>
  <div class="flex items-center whitespace-nowrap">
    <FilesLayoutToggle />
    <IconButton icon="ri-filter-line" margin />
    <TextButton icon="ri-upload-line" text="Upload" marginX />
    <TextButton icon="ri-folder-add-line" text="New folder" marginX />
  </div>
</div>
