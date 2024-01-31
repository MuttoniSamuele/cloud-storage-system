<script lang="ts">
  export let wfull = false;
</script>

<input
  type="text"
  class="px-2 py-1 rounded text-zinc-900 bg-zinc-500 dark:text-zinc-200 dark:bg-zinc-500
    {wfull ? 'w-full' : ''}"
/>
