<script lang="ts">
  export let title: string;
</script>

<section class="w-full">
  <h1 class="text-lg font-bold mb-2">
    {title}
  </h1>
  <div>
    <slot />
  </div>
</section>
