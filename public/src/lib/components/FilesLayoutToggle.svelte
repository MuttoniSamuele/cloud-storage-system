<script lang="ts">
  import { preferences } from "../stores/preferences";
  import IconButton from "./IconButton.svelte";
</script>

<IconButton
  icon={$preferences.filesLayout === "grid"
    ? "ri-layout-row-line"
    : "ri-layout-grid-line"}
  margin
  on:click={() => preferences.toggleFilesLayout()}
/>
