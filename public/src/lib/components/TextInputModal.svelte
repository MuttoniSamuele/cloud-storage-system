<script lang="ts">
  import { modalState, ModalState } from "../stores/modalState";
  import Modal from "./Modal.svelte";
  import TextButton from "./TextButton.svelte";
  import TextInput from "./TextInput.svelte";
</script>

<Modal
  title="Text input"
  size="sm"
  on:requestClose={() => modalState.set(ModalState.Closed)}
>
  <TextInput wfull />
  <div class="flex justify-end w-full mt-5 mb-1">
    <TextButton text="Confirm" />
  </div>
</Modal>
