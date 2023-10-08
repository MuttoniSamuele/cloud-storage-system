<script lang="ts">
  export let icon: string | null = null;
  export let text: string;
</script>

<button
  class="mx-2 px-2 py-1 rounded bg-indigo-600 text-zinc-100 hover:bg-indigo-700 focus-visible:outline-zinc-100"
  on:click
>
  <i class="{icon === null ? '' : icon} ri-lg" />
  {text}
</button>
