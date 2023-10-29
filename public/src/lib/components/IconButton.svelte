<script lang="ts">
  export let icon: string;
  export let small = false;
  export let margin = false;
</script>

<button
  class="text-zinc-500 hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-zinc-100
    {margin ? 'mx-2' : ''}"
  on:click
>
  <i class="{icon} {small ? 'ri-lg' : 'ri-xl'}" />
</button>
