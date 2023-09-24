<script lang="ts">
  import { RiCloseLine, RiEqualizerLine, RiSearchLine } from "svelte-remixicon";

  let value: string = "";
</script>

<div
  class="flex items-center w-1/3 h-8 rounded-md bg-zinc-200 text-zinc-700 dark:bg-zinc-700 dark:text-zinc-300"
>
  <button
    class="h-full px-2 rounded-l-md hover:bg-zinc-300 dark:hover:bg-zinc-600"
  >
    <RiSearchLine class="w-5" size="" />
  </button>
  <input
    type="search"
    class="block w-full h-full px-2 bg-transparent hover:bg-zinc-300 dark:hover:bg-zinc-600"
    placeholder="Search"
    bind:value
  />
  {#if value.length > 0}
    <button
      class="h-full px-1 hover:bg-zinc-300 dark:hover:bg-zinc-600"
      on:click={() => (value = "")}
    >
      <RiCloseLine class="w-5" size="" />
    </button>
  {/if}
  <button
    class="h-full px-2 rounded-r-md hover:bg-zinc-300 dark:hover:bg-zinc-600"
  >
    <RiEqualizerLine class="w-5" size="" />
  </button>
</div>
