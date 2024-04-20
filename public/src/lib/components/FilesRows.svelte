<script lang="ts">
  import FileRow from "./FileRow.svelte";
  import FileRowField from "./FileRowField.svelte";
  import File from "../logic/File";
  import type Folder from "../logic/Folder";
  import { cmpFileNames } from "../logic/fileUtils";
  import { createFilesEventDispatcher } from "./FilesGrid.svelte";

  export let files: File[];
  export let folders: Folder[];
  export let selectedFiles: Set<File | Folder>;
  export let showOwners = false;

  const dispatch = createFilesEventDispatcher();
</script>

<div class="relative">
  <div
    class="sticky top-0 flex items-center pl-5 pr-[44px] py-2 select-none border-b z-10 bg-zinc-100 dark:bg-zinc-800 border-zinc-300 dark:border-zinc-600"
  >
    <FileRowField size="large" isHeader>Name</FileRowField>
    <FileRowField size="small" secondary isHeader fixedMargin>
      Date added
    </FileRowField>
    {#if showOwners}
      <FileRowField size="medium" secondary isHeader>Owner</FileRowField>
    {/if}
  </div>
  {#each folders.sort(cmpFileNames) as folder}
    <FileRow
      name={folder.name}
      lastModified={folder.lastModified}
      selected={selectedFiles.has(folder)}
      isFolder
      on:click={() => dispatch("folderClick", folder)}
      on:dblclick={() => dispatch("folderDblClick", folder)}
      on:clickOutside={({ detail: e }) =>
        dispatch("folderClickOutside", { f: folder, e })}
      on:rightClick={({ detail: e }) =>
        dispatch("folderRightClick", { f: folder, e })}
      on:more={({ detail: e }) => dispatch("folderMore", { f: folder, e })}
    />
  {/each}
  {#each files.sort(cmpFileNames) as file}
    <FileRow
      name={file.name}
      fileType={file.fileType}
      lastModified={file.lastModified}
      selected={selectedFiles.has(file)}
      on:click={() => dispatch("fileClick", file)}
      on:dblclick={() => dispatch("fileDblClick", file)}
      on:clickOutside={({ detail: e }) =>
        dispatch("fileClickOutside", { f: file, e })}
      on:rightClick={({ detail: e }) =>
        dispatch("fileRightClick", { f: file, e })}
      on:more={({ detail: e }) => dispatch("fileMore", { f: file, e })}
    />
  {/each}
</div>
