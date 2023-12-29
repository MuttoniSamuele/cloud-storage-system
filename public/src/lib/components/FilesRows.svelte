<script lang="ts">
  import FileRow from "./FileRow.svelte";
  import FileRowField from "./FileRowField.svelte";
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
    fileClickOutside: { file: File; e: MouseEvent };
    folderClick: Folder;
    folderDblClick: Folder;
    folderClickOutside: { folder: Folder; e: MouseEvent };
    more: void;
  }>();
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
  {#each folders.sort(cmpFileNames) as folder}
    <FileRow
      name={folder.name}
      owner={showOwners ? folder.owner : null}
      lastModified={folder.lastModified}
      selected={selectedFiles.has(folder)}
      isFolder
      on:click={() => dispatch("folderClick", folder)}
      on:dblclick={() => dispatch("folderDblClick", folder)}
      on:clickOutside={({ detail: e }) =>
        dispatch("folderClickOutside", { folder, e })}
      on:more={() => dispatch("more")}
    />
  {/each}
  {#each files.sort(cmpFileNames) as file}
    <FileRow
      name={file.displayName}
      fileType={file.fileType}
      owner={showOwners ? file.owner : null}
      lastModified={file.lastModified}
      selected={selectedFiles.has(file)}
      on:click={() => dispatch("fileClick", file)}
      on:dblclick={() => dispatch("fileDblClick", file)}
      on:clickOutside={({ detail: e }) =>
        dispatch("fileClickOutside", { file, e })}
      on:more={() => dispatch("more")}
    />
  {/each}
</div>
