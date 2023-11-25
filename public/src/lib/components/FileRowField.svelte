<script lang="ts">
  export let size: "large" | "medium" | "small";
  export let secondary = false;
  export let isHeader = false;
  // Margin to apply to the second field in order to make up for the space occupied
  // by the file icon. (It can't be applied to the first field because it looks bad)
  export let fixedMargin = false;
</script>

<div
  class="basis-80 mr-auto whitespace-nowrap overflow-ellipsis overflow-hidden text-zinc-900 dark:text-zinc-200
    {isHeader ? 'font-bold' : ''}
    {size === 'large'
    ? 'basis-5/12'
    : size === 'medium'
      ? 'basis-1/4'
      : 'basis-1/6'}
    {secondary ? (isHeader ? '' : 'ml-auto') : isHeader ? '' : 'ml-5'}
    {fixedMargin ? 'ml-[41px]' : ''}"
>
  <slot />
</div>
