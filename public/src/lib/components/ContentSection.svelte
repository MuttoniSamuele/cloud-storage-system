<script lang="ts">
  export let top = false;
</script>

<section
  class="w-full p-4 bg-zinc-100 dark:bg-zinc-800
    {top ? 'rounded-tl-lg' : 'mt-2 flex-1'}"
>
  <slot />
</section>
