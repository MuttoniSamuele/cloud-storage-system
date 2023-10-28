<script lang="ts">
  import Breadcrumb from "./Breadcrumb.svelte";
  import FilesLayoutToggle from "./FilesLayoutToggle.svelte";
  import IconButton from "./IconButton.svelte";
  import Navigation from "./Navigation.svelte";
  import TextButton from "./TextButton.svelte";
</script>

<section class="flex justify-between">
  <div class="flex items-center">
    <Navigation />
    <IconButton icon="ri-restart-line ml-1 mr-2" small />
    <Breadcrumb />
  </div>
  <div class="flex items-center">
    <FilesLayoutToggle />
    <IconButton icon="ri-filter-line" margin />
    <TextButton icon="ri-upload-line" text="Upload" />
    <TextButton icon="ri-folder-add-line" text="New folder" />
  </div>
</section>
