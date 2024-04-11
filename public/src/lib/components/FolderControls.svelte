<script lang="ts">
  import { account } from "../stores/account";
  import { ModalState, modalState } from "../stores/modalState";
  import Breadcrumb from "./Breadcrumb.svelte";
  import FilesLayoutToggle from "./FilesLayoutToggle.svelte";
  // import IconButton from "./IconButton.svelte";
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
    <!-- TODO: Implement client-side filtering (by extension) -->
    <!-- <IconButton icon="ri-filter-line" margin /> -->
    <TextButton
      icon="ri-upload-line"
      text="Upload"
      marginX
      on:click={() => $account !== null && modalState.set(ModalState.Upload)}
    />
    <TextButton
      icon="ri-folder-add-line"
      text="New folder"
      marginX
      on:click={() => $account !== null && modalState.set(ModalState.TextInput)}
    />
  </div>
</div>
