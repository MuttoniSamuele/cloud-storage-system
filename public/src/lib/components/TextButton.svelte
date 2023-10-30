<script lang="ts">
  export let icon: string | null = null;
  export let text: string;
  export let marginX: boolean = false;
  export let marginY: boolean = false;
  export let wide: boolean = false;
</script>

<button
  class="px-2 py-1 bg-indigo-600 text-zinc-100 hover:bg-indigo-700 contrast-focus
    {wide ? 'w-64 font-bold rounded-md' : 'rounded'}
    {marginX ? 'mx-2' : ''} {marginY ? 'my-2' : ''}"
  on:click
>
  <i class="{icon === null ? '' : icon} ri-lg" />
  {text}
</button>
