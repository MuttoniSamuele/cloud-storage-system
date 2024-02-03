<script lang="ts">
  import { modalState, ModalState } from "../stores/modalState";
  import LoginModal from "./LoginModal.svelte";
  import TextInputModal from "./TextInputModal.svelte";
</script>

{#if $modalState === ModalState.Login}
  <LoginModal />
{:else if $modalState === ModalState.TextInput}
  <TextInputModal />
{/if}
