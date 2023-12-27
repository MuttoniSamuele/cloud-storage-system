<script lang="ts">
  import FileCell from "./FileCell.svelte";
  import File from "../logic/File";
  import type Folder from "../logic/Folder";
  import { cmpFileNames } from "../logic/fileUtils";
  import { createEventDispatcher } from "svelte";
  import type FileBase from "../logic/FileBase";

  export let files: File[];
  export let folders: Folder[];
  export let showOwners = false;

  const dispatch = createEventDispatcher<{
    fileSelect: File;
    folderSelect: Folder;
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
      isFolder
      on:select={() => dispatch("folderSelect", folder)}
      on:more={() => dispatch("more")}
    />
  {/each}
  {#each files.sort(cmpFileNames) as file}
    <FileCell
      name={file.displayName}
      fileType={file.fileType}
      owner={showOwners ? file.owner : null}
      on:select={() => dispatch("fileSelect", file)}
      on:more={() => dispatch("more")}
    />
  {/each}
</div>
