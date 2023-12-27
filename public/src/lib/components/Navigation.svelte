<script lang="ts">
  import { pathsHistory } from "../stores/pathsHistory";
  import IconButton from "./IconButton.svelte";
</script>

<div class="flex items-center">
  <IconButton
    icon="ri-arrow-left-line"
    on:click={() => pathsHistory.goBack()}
  />
  <IconButton
    icon="ri-arrow-right-line"
    on:click={() => pathsHistory.goForward()}
  />
</div>
