<script lang="ts">
  import { fileTypeToIcon, formatLastModified } from "../logic/fileUtils";
  import FileRowField from "./FileRowField.svelte";
  import IconButton from "./IconButton.svelte";
  import { clickOutside } from "../actions/clickOutside";
  import { createFileEventDispatcher } from "./FileCell.svelte";

  export let name: string;
  export let isFolder = false;
  export let fileType: string | null = null;
  export let owner: string | null = null;
  export let lastModified: string;
  export let selected = false;

  const dispatch = createFileEventDispatcher();
</script>

<li
  class="relative flex items-center px-5 py-2 select-none border-b border-zinc-300 dark:border-zinc-600
  {selected
    ? 'bg-zinc-300 dark:bg-zinc-600'
    : 'hover:bg-zinc-200 dark:hover:bg-zinc-700'}"
  title={name}
>
  <button
    class="absolute top-0 left-0 w-full h-full cursor-default"
    on:click
    on:contextmenu|preventDefault={(e) => dispatch("rightClick", e)}
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
    {formatLastModified(lastModified)}
  </FileRowField>
  {#if owner !== null}
    <FileRowField size="medium" secondary>
      {owner}
    </FileRowField>
  {/if}
  <div class="w-[24px] ml-auto"></div>
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div
    class="absolute right-4"
    on:contextmenu|preventDefault={(e) => dispatch("rightClick", e)}
  >
    <IconButton icon="ri-more-line" on:click={(e) => dispatch("more", e)} />
  </div>
</li>
