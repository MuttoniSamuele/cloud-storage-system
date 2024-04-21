<script lang="ts">
  import { modalState, ModalState } from "../stores/modalState";
  import Modal from "./Modal.svelte";
  import { selectedFiles } from "../stores/selectedFile";

  $: selectedFile = selectedFiles.getOne();
</script>

<Modal
  title={selectedFile === null ? "No file selected" : selectedFile.name}
  size="sm"
  on:requestClose={() => modalState.set(ModalState.Closed)}
>
  {#if selectedFile !== null}
    <div class="text-yellow-500 mb-2">
      <p>This file format is not supported yet.</p>
    </div>
  {/if}
</Modal>
