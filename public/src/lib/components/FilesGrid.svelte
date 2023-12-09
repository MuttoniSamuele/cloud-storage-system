<script lang="ts">
  import FileCell from "./FileCell.svelte";
  import File from "../logic/File";
  import type Folder from "../logic/Folder";
  import { cmpFileNames } from "../logic/fileUtils";

  export let files: File[];
  export let folders: Folder[];
  export let showOwners = false;
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
    />
  {/each}
  {#each files.sort(cmpFileNames) as file}
    <FileCell
      name={file.name}
      fileType={file.fileType}
      owner={showOwners ? file.owner : null}
    />
  {/each}
</div>
