<script lang="ts">
  import { pathsHistory } from "../stores/pathsHistory";
  import IconButton from "./IconButton.svelte";
</script>

<IconButton
  icon="ri-restart-line"
  small
  margin
  on:click={() => pathsHistory.refresh()}
/>
