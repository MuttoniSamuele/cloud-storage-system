<script lang="ts">
  import { onMount } from "svelte";

  let availableHeight: number | null = null;

  function handleWindowResize() {
    availableHeight = null;
  }

  onMount(() => {
    // Don't even ask me why, I don't know anymore
    availableHeight = null;
    window.addEventListener("resize", handleWindowResize);
    return () => {
      window.removeEventListener("resize", handleWindowResize);
    };
  });
</script>

<div
  class="h-full overflow-y-auto"
  bind:offsetHeight={availableHeight}
  on:scroll
>
  <div style="height: {availableHeight !== null ? availableHeight : 0}px;">
    <slot />
  </div>
</div>
