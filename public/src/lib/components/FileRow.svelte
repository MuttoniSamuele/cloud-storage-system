<script lang="ts">
  import FileRowField from "./FileRowField.svelte";
  import IconButton from "./IconButton.svelte";

  export let name: string;
  export let lastModified: Date | null = null;
  export let isFolder = false;
  export let owner: string | null = null;
</script>

<li
  class="flex items-center px-5 py-2 select-none border-b border-zinc-300 dark:border-zinc-600 hover:bg-zinc-200 dark:hover:bg-zinc-700"
>
  <i
    class="{isFolder
      ? 'ri-folder-3-fill text-indigo-400'
      : 'ri-file-2-fill text-zinc-400 dark:text-zinc-200'}
        ri-lg"
  />
  <FileRowField size="large">
    {name}
  </FileRowField>
  <FileRowField size="small" secondary>
    {lastModified !== null
      ? lastModified.toLocaleDateString("en-US", {
          year: "numeric",
          month: "2-digit",
          day: "2-digit",
        })
      : "--"}
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
