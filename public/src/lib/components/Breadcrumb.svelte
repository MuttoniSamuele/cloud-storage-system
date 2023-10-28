<script lang="ts">
  import { workingFolder } from "../stores/workingFolder";
</script>

{#if $workingFolder !== null}
  <div class="text-lg">
    {#each $workingFolder.rawPath as folder, i}
      <a
        href="/"
        class="px-2 py-1 rounded-md text-zinc-700 hover:bg-zinc-200 dark:text-zinc-300 dark:hover:bg-zinc-700"
      >
        {folder}
      </a>
      {#if i < $workingFolder.rawPath.length - 1}
        <i
          class="ri-arrow-right-s-line ri-1x -ml-1 text-zinc-400 dark:text-zinc-500"
        />
      {/if}
    {/each}
  </div>
{/if}
