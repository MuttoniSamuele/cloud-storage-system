<script lang="ts">
  import FileCell from "./FileCell.svelte";
  import File from "../logic/File";

  export let files: File[];
  export let showOwners = false;
</script>

<div
  class="grid gap-8 justify-center p-4"
  style="grid-template-columns: repeat(auto-fill, minmax(8rem, 1fr));"
>
  {#each files as file}
    <FileCell
      name={file.name}
      isFolder={file.isFolder}
      fileType={file.fileType}
      owner={showOwners ? file.owner : null}
    />
  {/each}
</div>
