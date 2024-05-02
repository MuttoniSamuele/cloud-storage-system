<script lang="ts">
  import { formatBytes } from "../logic/fileUtils";
  import { ModalState, modalState } from "../stores/modalState";
  import IconButton from "./IconButton.svelte";
  import Loader from "./Loader.svelte";

  export let usedSpace: number | null = null;
  export let totalSpace: number;
  export let loading = false;
  export let unknown = false;
  export let hideTotal = false;
</script>

<div class="w-full text-zinc-700 dark:text-zinc-300">
  <div class="w-4/5 m-auto">
    <div class="flex justify-between mb-2 px-2">
      <div class="flex items-center">
        {#if loading}
          <Loader small />
        {:else if unknown || usedSpace === null}
          <span>??</span>
        {:else}
          <span>{formatBytes(usedSpace, 1)}</span>
        {/if}
        {#if !hideTotal}
          <span class="mx-1">/</span>
          <span>{formatBytes(totalSpace, 1)}</span>
        {/if}
      </div>
      <IconButton
        icon="ri-information-line"
        small
        on:click={() => modalState.set(ModalState.Cloud)}
      />
    </div>
    <div class="h-2 rounded-full bg-zinc-300 dark:bg-zinc-700">
      <div
        class="h-full rounded-full bg-indigo-600"
        style="width: {(usedSpace === null ? 0 : usedSpace * 100) /
          totalSpace}%;"
      />
    </div>
  </div>
</div>
