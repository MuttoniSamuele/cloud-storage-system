<script lang="ts">
  import { modalState, ModalState } from "../stores/modalState";
  import Modal from "./Modal.svelte";
  import API from "../logic/api";
  import { selectedFiles } from "../stores/selectedFile";
  import Loader from "./Loader.svelte";

  $: selectedFile = selectedFiles.getOne();

  async function getFileContent(fileId: number): Promise<string> {
    const [_fileName, blob] = await API.getFileContent(fileId);
    return await blob.text();
  }
</script>

<Modal
  title={selectedFile === null ? "No file selected" : selectedFile.name}
  size="fill"
  on:requestClose={() => modalState.set(ModalState.Closed)}
>
  {#if selectedFile !== null}
    {#await getFileContent(selectedFile.id)}
      <Loader />
    {:then content}
      <pre>{content}</pre>
    {/await}
  {/if}
</Modal>
