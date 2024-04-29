<script lang="ts">
  export let icon: string | null = null;
  export let text: string;
  export let marginX = false;
  export let marginY = false;
  export let wide = false;
  export let isLink = false;
  export let disabled = false;
  export let slot = false;
</script>

<button
  class=" contrast-focus
    {isLink
    ? 'bg-transparent text-zinc-600 hover:text-zinc-800 dark:text-zinc-400 dark:hover:text-zinc-200'
    : 'px-2 py-1 bg-indigo-600 hover:bg-indigo-700 disabled:bg-indigo-500 text-zinc-100'}
    {wide ? 'w-64 font-bold rounded-md' : 'rounded'}
    {marginX ? 'mx-2' : ''} {marginY ? 'my-2' : ''}"
  on:click
  {disabled}
>
  <i class="{icon === null ? '' : icon} ri-lg" />
  {text}
  {#if slot}
    <slot />
  {/if}
</button>
