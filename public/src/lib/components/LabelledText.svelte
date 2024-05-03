<script lang="ts">
  export let label: string;
  export let text: string;
  export let showTitle = false;
</script>

<div class="relative w-full mb-2">
  <div class="absolute left-0 bottom-0 text-sm dark:text-zinc-400">
    {label}
  </div>
  <div
    class="ml-20 text-base whitespace-nowrap text-ellipsis overflow-hidden dark:text-zinc-100"
    title={showTitle ? text : undefined}
  >
    {text}
  </div>
</div>
