<script lang="ts" context="module">
  import { createEventDispatcher } from "svelte";

  // This is stuff that FilesGrid and FilesRows have in common.

  export interface ClickPositionEvent<T = File | Folder> {
    f: T;
    e: MouseEvent;
  }

  export function createFilesEventDispatcher() {
    return createEventDispatcher<{
      fileClick: File;
      fileDblClick: File;
      fileClickOutside: ClickPositionEvent<File>;
      fileRightClick: ClickPositionEvent<File>;
      fileMore: ClickPositionEvent<File>;
      folderClick: Folder;
      folderDblClick: Folder;
      folderClickOutside: ClickPositionEvent<Folder>;
      folderRightClick: ClickPositionEvent<Folder>;
      folderMore: ClickPositionEvent<Folder>;
    }>();
  }
</script>

<script lang="ts">
  import FileCell from "./FileCell.svelte";
  import File from "../logic/File";
  import type Folder from "../logic/Folder";
  import { cmpFileNames } from "../logic/fileUtils";

  export let files: File[];
  export let folders: Folder[];
  export let selectedFiles: Set<File | Folder>;
  export let showOwners = false;

  const dispatch = createFilesEventDispatcher();
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
      on:clickOutside={({ detail: e }) =>
        dispatch("folderClickOutside", { f: folder, e })}
      on:rightClick={({ detail: e }) =>
        dispatch("folderRightClick", { f: folder, e })}
      on:more={({ detail: e }) => dispatch("folderMore", { f: folder, e: e })}
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
      on:clickOutside={({ detail: e }) =>
        dispatch("fileClickOutside", { f: file, e })}
      on:rightClick={({ detail: e }) =>
        dispatch("fileRightClick", { f: file, e })}
      on:more={({ detail: e }) => dispatch("fileMore", { f: file, e: e })}
    />
  {/each}
</div>
