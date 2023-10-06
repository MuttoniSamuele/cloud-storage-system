<script lang="ts">
  import IconButton from "./IconButton.svelte";

  export let usedSpace: number;
  export let totalSpace: number;
</script>

<div class="w-full text-zinc-700 dark:text-zinc-300">
  <div class="w-4/5 m-auto">
    <div class="flex justify-between mb-2 px-2">
      <span>{usedSpace} GB / {totalSpace} GB</span>
      <IconButton icon="ri-information-line" small />
    </div>
    <div class="h-2 rounded-full bg-zinc-300 dark:bg-zinc-700">
      <div
        class="h-full rounded-full bg-indigo-600"
        style="width: {(usedSpace * 100) / totalSpace}%;"
      />
    </div>
  </div>
</div>
