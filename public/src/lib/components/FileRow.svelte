<script lang="ts">
  import { fileTypeToIcon, type FileType } from "../logic/fileUtils";
  import FileRowField from "./FileRowField.svelte";
  import IconButton from "./IconButton.svelte";
  import { clickOutside } from "../actions/clickOutside";
  import { createFileEventDispatcher } from "./FileCell.svelte";

  export let name: string;
  export let isFolder = false;
  export let fileType: FileType | null = null;
  export let owner: string | null = null;
  export let lastModified: number;
  export let selected = false;

  const dispatch = createFileEventDispatcher();

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
  class="relative flex items-center px-5 py-2 select-none border-b border-zinc-300 dark:border-zinc-600
  {selected
    ? 'bg-zinc-300 dark:bg-zinc-600'
    : 'hover:bg-zinc-200 dark:hover:bg-zinc-700'}"
>
  <button
    class="absolute top-0 left-0 w-full h-full cursor-default"
    on:click
    on:contextmenu|preventDefault
    on:dblclick
    use:clickOutside={(e) => dispatch("clickOutside", e)}
  ></button>
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
  <div class="w-[24px] ml-auto"></div>
  <div class="absolute right-4">
    <IconButton icon="ri-more-line" on:click={() => dispatch("more")} />
  </div>
</li>
