<script lang="ts">
  import { formatBytes } from "../logic/fileUtils";

  export let usedSpaces: [string, number][];
  export let totUsedSpace: number;
  export let totalSpace: number;

  const COLORS = ["bg-blue-500", "bg-green-500", "bg-yellow-500", "bg-red-500"];
</script>

<div class="inline-block mb-1 ml-2">
  <span>{formatBytes(totUsedSpace)}</span>
  <span class="text-sm">/ {formatBytes(totalSpace)}</span>
</div>

<div class="flex w-full h-3 mb-4 rounded-full bg-zinc-500">
  {#each usedSpaces as [name, space], i}
    <div
      class="h-full {COLORS[i % COLORS.length]}
      {i === 0 ? 'rounded-l-full' : ''}
      {i === usedSpaces.length - 1 ? 'rounded-r-full' : ''}"
      style="width: {(space * 100) / totalSpace}%;"
      title={name}
    />
  {/each}
</div>

{#if usedSpaces !== null}
  <div class="flex flex-wrap">
    {#each usedSpaces as [name, space], i}
      <div class="flex items-center mr-6">
        <div
          class="inline-block w-2 h-2 mr-2 rounded-full
          {COLORS[i % COLORS.length]}"
        ></div>
        <span>
          <span class="font-bold">{name}</span>: {formatBytes(space, 2)}</span
        >
      </div>
    {/each}
  </div>
{/if}
