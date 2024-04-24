<script lang="ts">
  import API from "../logic/api";
  import { modalState, ModalState } from "../stores/modalState";
  import Modal from "./Modal.svelte";
  import TextButton from "./TextButton.svelte";
  import IconButton from "./IconButton.svelte";
  import { pathsHistory } from "../stores/pathsHistory";
  import Loader from "./Loader.svelte";
  import { formatBytes } from "../logic/fileUtils";
  import { fileChange } from "../stores/fileChange";

  let inputElem: HTMLInputElement | null = null;
  let selectedFile: File | null = null;
  let errorMessage: string | null = null;
  let isUploading = false;

  // Set errorMessage to null every time selectedFile changes
  $: if (selectedFile || true) {
    errorMessage = null;
  }

  async function handleUpload(): Promise<void> {
    if (selectedFile === null) {
      return;
    }
    const curPath = $pathsHistory.paths[$pathsHistory.index];
    if (curPath === null) {
      errorMessage = "You can't upload a file here.";
      return;
    }
    isUploading = true;
    try {
      await API.upload(selectedFile, curPath.rawPath.reverse()[0].id);
    } catch (e) {
      if (e instanceof API.ApiError) {
        errorMessage = e.message;
        return;
      }
      errorMessage = "The file can't be larger than 10 MB.";
      throw e;
    } finally {
      isUploading = false;
    }
    modalState.set(ModalState.Closed);
    fileChange.setFile(selectedFile.name);
    pathsHistory.refresh();
  }
</script>

<Modal
  title="Upload files"
  size="sm"
  on:requestClose={() => modalState.set(ModalState.Closed)}
>
  {#if selectedFile !== null}
    <div
      class="relative flex px-3 py-2 mb-6 rounded-lg dark:bg-zinc-600"
      title={selectedFile.name}
    >
      <span
        class="max-w-[65%] mr-2 whitespace-nowrap text-ellipsis overflow-hidden font-bold"
      >
        {selectedFile.name}
      </span>
      <span>
        ({formatBytes(selectedFile.size)})
      </span>
      <div class="absolute right-2">
        {#if isUploading}
          <Loader />
        {:else}
          <IconButton
            icon="ri-close-line"
            small
            on:click={() => (selectedFile = null)}
          />
        {/if}
      </div>
    </div>
  {/if}
  <input
    bind:this={inputElem}
    type="file"
    name="f"
    class="hidden"
    on:change={() => inputElem?.files && (selectedFile = inputElem.files[0])}
  />

  {#if errorMessage !== null}
    <div class="text-red-500 mt-2">
      {errorMessage}
    </div>
  {/if}

  <div class="flex flex-col items-center w-full mt-1 mb-3">
    {#if selectedFile === null}
      <TextButton text="Browse" wide on:click={() => inputElem?.click()} />
    {:else}
      <TextButton text="Upload" wide on:click={handleUpload} />
    {/if}
  </div>
</Modal>
