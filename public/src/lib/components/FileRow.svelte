<script lang="ts">
  import { fileTypeToIcon, type FileType } from "../logic/fileUtils";
  import FileRowField from "./FileRowField.svelte";
  import IconButton from "./IconButton.svelte";

  export let name: string;
  export let isFolder = false;
  export let fileType: FileType | null = null;
  export let owner: string | null = null;
  export let lastModified: number;

  function formatLastModified(): string {
    return new Intl.DateTimeFormat(navigator.language, {
      day: "2-digit",
      month: "2-digit",
      year: "numeric",
      hour: "2-digit",
      minute: "2-digit",
    }).format(new Date(lastModified * 1000));
  }
</script>

<li
  class="flex items-center px-5 py-2 select-none border-b border-zinc-300 dark:border-zinc-600 hover:bg-zinc-200 dark:hover:bg-zinc-700"
>
  <i
    class="{isFolder
      ? 'ri-folder-3-fill text-indigo-400'
      : `${fileTypeToIcon(
          fileType || 'Unsupported',
        )} text-zinc-400 dark:text-zinc-200`}
        ri-lg"
  />
  <FileRowField size="large">
    {name}
  </FileRowField>
  <FileRowField size="small" secondary>
    {formatLastModified()}
  </FileRowField>
  {#if owner !== null}
    <FileRowField size="medium" secondary>
      {owner}
    </FileRowField>
  {/if}
  <div class="ml-auto">
    <IconButton icon="ri-more-line" />
  </div>
</li>
