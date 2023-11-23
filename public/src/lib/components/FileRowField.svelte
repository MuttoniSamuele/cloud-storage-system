<script lang="ts">
  export let size: "large" | "medium" | "small";
  export let secondary = false;
</script>

<div
  class="basis-80 whitespace-nowrap overflow-ellipsis overflow-hidden
    {size === 'large'
    ? 'basis-5/12'
    : size === 'medium'
      ? 'basis-1/4'
      : 'basis-1/6'}
    {secondary
    ? 'mx-auto text-zinc-800 dark:text-zinc-300'
    : 'ml-5 mr-auto text-zinc-900 dark:text-zinc-200'}"
>
  <slot />
</div>
