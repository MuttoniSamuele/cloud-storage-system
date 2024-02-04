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
      fileContextMenu: ClickPositionEvent<File>;
      folderClick: Folder;
      folderDblClick: Folder;
      folderClickOutside: ClickPositionEvent<Folder>;
      folderContextMenu: ClickPositionEvent<Folder>;
      more: void;
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
      on:contextmenu={(e) => dispatch("folderContextMenu", { f: folder, e })}
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
      on:clickOutside={({ detail: e }) =>
        dispatch("fileClickOutside", { f: file, e })}
      on:contextmenu={(e) => dispatch("fileContextMenu", { f: file, e })}
      on:more={() => dispatch("more")}
    />
  {/each}
</div>
