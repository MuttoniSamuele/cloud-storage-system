<script lang="ts">
  import { modalState, ModalState } from "../stores/modalState";
  import LoginModal from "./LoginModal.svelte";
  import SignupModal from "./SignupModal.svelte";
  import TextInputModal from "./TextInputModal.svelte";
</script>

{#if $modalState === ModalState.Login}
  <LoginModal />
{:else if $modalState === ModalState.Signup}
  <SignupModal />
{:else if $modalState === ModalState.TextInput}
  <TextInputModal />
{/if}
