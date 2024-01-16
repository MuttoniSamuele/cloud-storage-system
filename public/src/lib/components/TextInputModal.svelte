<script lang="ts">
  import Modal from "./Modal.svelte";
  import { modalState, ModalState } from "../stores/modalState";
</script>

<Modal
  title="Text input"
  size="sm"
  on:requestClose={() => modalState.set(ModalState.Closed)}
>
  <input class="w-full" />
</Modal>
