<script lang="ts">
  import { modalState, ModalState } from "../stores/modalState";
  import LoginModal from "./LoginModal.svelte";
  import NewFolderModal from "./NewFolderModal.svelte";
  import SignupModal from "./SignupModal.svelte";
  import UploadModal from "./UploadModal.svelte";
</script>

{#if $modalState === ModalState.Login}
  <LoginModal />
{:else if $modalState === ModalState.Signup}
  <SignupModal />
{:else if $modalState === ModalState.Upload}
  <UploadModal />
{:else if $modalState === ModalState.NewFolder}
  <NewFolderModal />
{/if}
