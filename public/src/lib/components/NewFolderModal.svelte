<script lang="ts">
  import API from "../logic/api";
  import { ModalState, modalState } from "../stores/modalState";
  import { getCurrentPath, pathsHistory } from "../stores/pathsHistory";
  import TextInputModal from "./TextInputModal.svelte";

  let errorMessage: string | null = null;

  $: currentPath = getCurrentPath($pathsHistory);

  async function handleConfirm({ detail: name }: CustomEvent<string>) {
    if (currentPath === null) {
      errorMessage = "You can't create a folder here.";
      return;
    }
    try {
      await API.newFolder(currentPath?.lastId, name);
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

<TextInputModal title="New folder" {errorMessage} on:confirm={handleConfirm} />
