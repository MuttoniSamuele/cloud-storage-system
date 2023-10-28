<script lang="ts">
  import IconButton from "./IconButton.svelte";
  let isGrid = false;
</script>

<IconButton
  icon={isGrid ? "ri-layout-row-line" : "ri-layout-grid-line"}
  margin
  on:click={() => (isGrid = !isGrid)}
/>
