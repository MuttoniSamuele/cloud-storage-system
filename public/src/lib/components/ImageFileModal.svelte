<script lang="ts">
  import { modalState, ModalState } from "../stores/modalState";
  import Modal from "./Modal.svelte";
  import API from "../logic/api";
  import { selectedFiles } from "../stores/selectedFile";
  import Loader from "./Loader.svelte";
  import { onDestroy } from "svelte";

  let image: HTMLImageElement | null = null;
  $: selectedFile = selectedFiles.getOne();

  async function loadImageFile(fileId: number): Promise<void> {
    const [_fileName, blob] = await API.getFileContent(fileId);
    image = new Image();
    image.src = URL.createObjectURL(blob);
    image.onerror = function () {
      URL.revokeObjectURL(this.src);
      image = null;
    };
  }

  onDestroy(() => {
    if (image === null) {
      return;
    }
    URL.revokeObjectURL(image.src);
  });
</script>

<Modal
  title={selectedFile === null ? "No file selected" : selectedFile.name}
  size={image === null ? "sm" : "adapt"}
  scrollable={false}
  noContainer
  on:requestClose={() => modalState.set(ModalState.Closed)}
>
  {#if selectedFile !== null}
    {#await loadImageFile(selectedFile.id)}
      <div class="flex justify-center items-center w-full h-full mt-5 mb-2">
        <Loader />
      </div>
    {:then}
      {#if image === null}
        <div class="mt-5 mb-2 text-red-500">
          <p>Failed to load the image.</p>
        </div>
      {:else}
        <img src={image.src} alt={selectedFile.name} class="h-full mt-5 mb-1" />
      {/if}
    {/await}
  {/if}
</Modal>
