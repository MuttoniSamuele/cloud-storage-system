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
  scrollable
  on:requestClose={() => modalState.set(ModalState.Closed)}
>
  {#if selectedFile !== null}
    {#await getFileContent(selectedFile.id)}
      <div class="flex justify-center items-center w-full h-full">
        <Loader />
      </div>
    {:then content}
      <pre>{content}</pre>
    {:catch}
      <div class="text-red-500">
        <p>Failed to load the text file.</p>
      </div>
    {/await}
  {/if}
</Modal>
