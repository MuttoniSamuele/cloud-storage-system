<script lang="ts">
  import { modalState, ModalState } from "../stores/modalState";
  import ImageFileModal from "./ImageFileModal.svelte";
  import LoginModal from "./LoginModal.svelte";
  import NewFolderModal from "./NewFolderModal.svelte";
  import CloudModal from "./CloudModal.svelte";
  import RenameModal from "./RenameModal.svelte";
  import SignupModal from "./SignupModal.svelte";
  import TextFileModal from "./TextFileModal.svelte";
  import UnsupportedFileModal from "./UnsupportedFileModal.svelte";
  import UploadModal from "./UploadModal.svelte";
  import ConfirmationModal from "./ConfirmationModal.svelte";
  import PropertiesModal from "./PropertiesModal.svelte";
</script>

{#if $modalState === ModalState.Login}
  <LoginModal />
{:else if $modalState === ModalState.Signup}
  <SignupModal />
{:else if $modalState === ModalState.Upload}
  <UploadModal />
{:else if $modalState === ModalState.NewFolder}
  <NewFolderModal />
{:else if $modalState === ModalState.Rename}
  <RenameModal />
{:else if $modalState === ModalState.TextFile}
  <TextFileModal />
{:else if $modalState === ModalState.ImageFile}
  <ImageFileModal />
{:else if $modalState === ModalState.UnsupportedFile}
  <UnsupportedFileModal />
{:else if $modalState === ModalState.Cloud}
  <CloudModal />
{:else if $modalState === ModalState.Confirmation}
  <ConfirmationModal />
{:else if $modalState === ModalState.Properties}
  <PropertiesModal />
{/if}
