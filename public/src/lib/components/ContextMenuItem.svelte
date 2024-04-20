<script lang="ts">
  export let icon: string;
  export let text: string;
</script>

<li
  class="first:rounded-t-lg last:rounded-b-lg bg-zinc-100 hover:bg-zinc-200 text-zinc-700 dark:bg-zinc-800 dark:hover:bg-zinc-700 dark:text-zinc-300 hover:"
>
  <button class="w-full py-1 pr-16 text-start" on:click>
    <i class="{icon} ml-3 mr-2"></i>
    {text}
  </button>
</li>
