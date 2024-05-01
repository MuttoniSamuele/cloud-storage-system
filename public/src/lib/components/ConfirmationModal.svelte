<script lang="ts">
  import API from "../logic/api";
  import { modalState, ModalState } from "../stores/modalState";
  import Modal from "./Modal.svelte";
  import TextButton from "./TextButton.svelte";
</script>

<Modal
  title="Delete your account?"
  size="sm"
  on:requestClose={() => modalState.set(ModalState.Cloud)}
>
  <p class="mb-4">Are you sure you want to delete your account?</p>
  <div class="inline-block mr-2">
    <TextButton
      text="Delete"
      dangerous
      on:click={async () => await API.deleteMe()}
    />
  </div>
  <TextButton
    text="Cancel"
    secondary
    on:click={() => modalState.set(ModalState.Cloud)}
  />
</Modal>
