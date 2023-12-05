<script lang="ts">
  import FileRow from "./FileRow.svelte";
  import FileRowField from "./FileRowField.svelte";
  import File from "../logic/File";

  export let files: File[];
  export let showOwners = false;
</script>

<div class="relative">
  <div
    class="sticky top-0 flex items-center pl-5 pr-[44px] py-2 select-none border-b bg-zinc-100 dark:bg-zinc-800 border-zinc-300 dark:border-zinc-600"
  >
    <FileRowField size="large" isHeader>Name</FileRowField>
    <FileRowField size="small" secondary isHeader fixedMargin>
      Last modified
    </FileRowField>
    {#if showOwners}
      <FileRowField size="medium" secondary isHeader>Owner</FileRowField>
    {/if}
  </div>
  {#each files as file}
    <FileRow
      name={file.name}
      isFolder={file.isFolder}
      fileType={file.fileType}
      owner={showOwners ? file.owner : null}
      lastModified={file.lastModified}
    />
  {/each}
</div>
