<script lang="ts">
  import FileCell from "./FileCell.svelte";
  import File from "../logic/File";
  import type Folder from "../logic/Folder";
  import { cmpFileNames } from "../logic/fileUtils";
  import { createEventDispatcher } from "svelte";

  export let files: File[];
  export let folders: Folder[];
  export let selectedFiles: Set<File | Folder>;
  export let showOwners = false;

  const dispatch = createEventDispatcher<{
    fileClick: File;
    fileDblClick: File;
    folderClick: Folder;
    folderDblClick: Folder;
    more: void;
  }>();
</script>

<div
  class="grid gap-8 justify-center p-4"
  style="grid-template-columns: repeat(auto-fill, minmax(8rem, 1fr));"
>
  {#each folders.sort(cmpFileNames) as folder}
    <FileCell
      name={folder.name}
      owner={showOwners ? folder.owner : null}
      selected={selectedFiles.has(folder)}
      isFolder
      on:click={() => dispatch("folderClick", folder)}
      on:dblclick={() => dispatch("folderDblClick", folder)}
      on:more={() => dispatch("more")}
    />
  {/each}
  {#each files.sort(cmpFileNames) as file}
    <FileCell
      name={file.displayName}
      fileType={file.fileType}
      selected={selectedFiles.has(file)}
      owner={showOwners ? file.owner : null}
      on:click={() => dispatch("fileClick", file)}
      on:dblclick={() => dispatch("fileDblClick", file)}
      on:more={() => dispatch("more")}
    />
  {/each}
</div>
