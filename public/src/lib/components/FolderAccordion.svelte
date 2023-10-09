<script lang="ts">
  import type Path from "../logic/Path";
  import { account } from "../stores/account";
  import { workingFolder } from "../stores/workingFolder";
  import FolderTree from "./FolderTree.svelte";

  export let icon: string | null = null;
  export let displayName: string;
  export let path: Path;
  export let droppable = true;
  export let level = 0;

  let collapsed = true;

  $: isSelected = $workingFolder === null ? false : $workingFolder.cmp(path);
  // Check if it contains the selected folder and it isn't directly visible
  $: containsSelected =
    isSelected || !collapsed || $workingFolder === null
      ? false
      : $workingFolder.contains(path);

  function handleSelect(): void {
    if ($account !== null) {
      workingFolder.change(path.clone());
    }
  }
</script>

<div
  class="h-8 relative rounded-lg whitespace-nowrap select-none text-zinc-700 dark:text-zinc-300
    {isSelected
    ? 'bg-indigo-600 bg-opacity-70'
    : containsSelected
    ? 'bg-zinc-300 hover:bg-zinc-400 dark:bg-zinc-600 dark:hover:bg-zinc-500'
    : 'hover:bg-zinc-200 dark:hover:bg-zinc-700'}
    {level === 0 ? 'mt-3' : ''}"
>
  <button
    class="absolute top-1/2 -translate-y-1/2 w-full h-full text-start
      {isSelected ? 'contrast-focus' : ''}"
    on:click={handleSelect}
  >
    <span class="offset-folder-by-level" style="--level: {level};">
      <i class="{icon === null ? '' : icon} ri-lg" />
      {displayName}
    </span>
  </button>
  {#if droppable}
    <button
      class="absolute top-1/2 -translate-y-1/2 offset-arrow-by-level
        {isSelected ? 'contrast-focus' : ''}"
      style="--level: {level};"
      on:click={() => (collapsed = !collapsed)}
    >
      <!-- TODO: transition-transform -->
      <div class={collapsed ? "rotate-0" : "rotate-90"}>
        <i class="ri-arrow-drop-right-fill ri-xl" />
      </div>
    </button>
  {/if}
</div>
{#if droppable}
  <FolderTree level={level + 1} {collapsed} path={path.clone()} />
{/if}

<style>
  .offset-folder-by-level {
    margin-left: calc(1.6rem + var(--level) * 1rem);
  }

  .offset-arrow-by-level {
    left: calc(0.3rem + var(--level) * 1rem);
  }
</style>
