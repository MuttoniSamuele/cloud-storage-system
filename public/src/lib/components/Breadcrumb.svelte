<script lang="ts">
  import { workingFolder } from "../stores/workingFolder";

  let containerWidth: number | null = null;
  let innerWidth: number | null = null;
  let fits: boolean = true;
  $: if (containerWidth && innerWidth) {
    console.log(innerWidth, containerWidth);
    fits = innerWidth < containerWidth;
  }
</script>

{#if $workingFolder !== null}
  <div
    class="relative w-full h-full overflow-hidden"
    bind:offsetWidth={containerWidth}
  >
    <div
      class="absolute top-1/2 -translate-y-1/2 text-lg
        {fits ? 'left-0' : 'right-0'}"
      bind:offsetWidth={innerWidth}
    >
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
  </div>
{/if}
