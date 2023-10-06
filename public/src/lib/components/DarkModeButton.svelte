<script lang="ts">
  import IconButton from "./IconButton.svelte";
  import { theme } from "../stores/theme";
</script>

<IconButton
  icon={$theme === "dark" ? "ri-moon-line" : "ri-sun-line"}
  margin
  on:click={theme.toggle}
/>
