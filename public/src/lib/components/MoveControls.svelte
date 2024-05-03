<script lang="ts">
  import File from "../logic/File";
  import Folder from "../logic/Folder";
  import API from "../logic/api";
  import { fileMove } from "../stores/fileMove";
  import { getCurrentPath, pathsHistory } from "../stores/pathsHistory";
  import TextButton from "./TextButton.svelte";

  export let file: File | Folder;

  $: isFolder = file instanceof Folder;
  $: currentPath = getCurrentPath($pathsHistory);

  $: if (currentPath === null) {
    // If the current path is null, you can't move the file
    fileMove.cancel();
  } else if (file instanceof Folder) {
    // If the file is a folder, you can't move it to a subfolder of itself
    currentPath.rawPath.forEach((path) => {
      if (path.id === file.id) {
        fileMove.cancel();
      }
    });
  }
</script>

<TextButton
  icon="ri-folder-transfer-line"
  text="Move here"
  marginX
  on:click={async () => {
    if (currentPath === null) {
      return;
    }
    await API.moveFile(
      file.id,
      currentPath.rawPath[currentPath.rawPath.length - 1].id,
      isFolder,
    );
    fileMove.cancel();
    pathsHistory.refresh();
  }}
/>
<TextButton text="Cancel" marginX on:click={() => fileMove.cancel()} />
