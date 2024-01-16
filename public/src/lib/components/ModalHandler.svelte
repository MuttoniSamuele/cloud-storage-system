<script lang="ts">
  import { modalState, ModalState } from "../stores/modalState";
  import TextInputModal from "./TextInputModal.svelte";
</script>

{#if $modalState === ModalState.TextInput}
  <TextInputModal />
{/if}
