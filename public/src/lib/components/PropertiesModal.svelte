<script lang="ts">
  import File from "../logic/File";
  import Folder from "../logic/Folder";
  import API from "../logic/api";
  import { formatBytes, formatLastModified } from "../logic/fileUtils";
  import { modalState, ModalState } from "../stores/modalState";
  import { getCurrentPath, pathsHistory } from "../stores/pathsHistory";
  import { selectedFiles } from "../stores/selectedFile";
  import LabelledText from "./LabelledText.svelte";
  import Modal from "./Modal.svelte";

  $: currentPath = getCurrentPath($pathsHistory);
  $: selectedFile = selectedFiles.getOne();
  $: isFolder = selectedFile instanceof Folder;
</script>

<Modal
  title="Properties"
  size="sm"
  on:requestClose={() => modalState.set(ModalState.Closed)}
>
  {#if selectedFile !== null && currentPath !== null}
    <LabelledText label="Name" text={selectedFile.name} showTitle />
    <LabelledText label="Path" text={currentPath.toString()} showTitle />
    {#if selectedFile instanceof File}
      <LabelledText label="Type" text={selectedFile.fileType ?? "Unknown"} />
      <LabelledText label="Size" text={formatBytes(selectedFile.size)} />
    {:else}
      {#await API.getFolderSize(selectedFile.id) then size}
        <LabelledText label="Size" text={formatBytes(size)} />
      {/await}
    {/if}
    <LabelledText
      label={isFolder ? "Created" : "Uploaded"}
      text={formatLastModified(selectedFile.lastModified)}
    />
  {/if}
</Modal>
