<script lang="ts">
  export let icon: string;
</script>

<button class="h-min mr-3" on:click>
  <i
    class="{icon} ri-xl text-zinc-500 hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-zinc-100"
  />
</button>
