<script lang="ts">
  import Folder from "../logic/Folder";
  import API from "../logic/api";
  import { ModalState, modalState } from "../stores/modalState";
  import { getCurrentPath, pathsHistory } from "../stores/pathsHistory";
  import { selectedFiles } from "../stores/selectedFile";
  import TextInputModal from "./TextInputModal.svelte";

  $: currentPath = getCurrentPath($pathsHistory);
  $: selectedFile = selectedFiles.getOne();
  $: isFolder = selectedFile instanceof Folder;
  let errorMessage: string | null = null;

  async function handleConfirm({ detail: name }: CustomEvent<string>) {
    if (selectedFile === null) {
      errorMessage = "No file/folder selected";
      return;
    }
    if (currentPath === null) {
      errorMessage = "You can't create a folder here.";
      return;
    }
    try {
      if (isFolder) {
        // TODO: await API.renameFolder(selectedFile.id, name);
      } else {
        await API.renameFile(selectedFile.id, name);
      }
    } catch (e) {
      if (e instanceof API.ApiError) {
        errorMessage = e.message;
        return;
      }
      throw e;
    }
    modalState.set(ModalState.Closed);
    pathsHistory.refresh();
  }
</script>

<TextInputModal
  title="Rename {isFolder ? 'folder' : 'file'}"
  {errorMessage}
  on:confirm={handleConfirm}
/>
