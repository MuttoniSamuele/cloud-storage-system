<script lang="ts">
  import { workingFolder } from "../stores/workingFolder";
  import FolderContent from "./FolderContent.svelte";
  import FolderControls from "./FolderControls.svelte";
</script>

<main
  class="flex flex-col w-full h-full rounded-tl-lg bg-zinc-100 text-zinc-700 dark:bg-zinc-800 dark:text-zinc-300"
>
  {#if $workingFolder !== null}
    <FolderControls />
    <FolderContent />
  {:else}
    <!-- TODO: -->
    <span>Please login</span>
  {/if}
</main>
