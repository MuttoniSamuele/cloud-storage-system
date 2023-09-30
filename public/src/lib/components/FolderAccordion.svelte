<script lang="ts">
  import FolderTree from "./FolderTree.svelte";

  export let icon: string | null = null;
  export let name: string;
  export let droppable = true;
  export let level = 0;

  let collapsed = true;
</script>

<div
  class="h-8 relative whitespace-nowrap select-none text-zinc-700 dark:text-zinc-300 hover:bg-zinc-200 dark:hover:bg-zinc-700
    {level === 0 ? 'mt-3' : ''}"
>
  <button class="absolute top-1/2 -translate-y-1/2 w-full h-full text-start">
    <span class="offset-folder-by-level" style="--level: {level};">
      <i class="{icon === null ? '' : icon} ri-lg" />
      {name}
    </span>
  </button>
  {#if droppable}
    <button
      class="absolute top-1/2 -translate-y-1/2 offset-arrow-by-level"
      style="--level: {level};"
      on:click={() => (collapsed = !collapsed)}
    >
      <i class="ri-arrow-drop-right-fill ri-xl" />
    </button>
  {/if}
</div>
{#if !collapsed}
  <FolderTree level={level + 1} />
{/if}

<style>
  .offset-folder-by-level {
    margin-left: calc(1.6rem + var(--level) * 1rem);
  }

  .offset-arrow-by-level {
    left: calc(0.3rem + var(--level) * 1rem);
  }
</style>
