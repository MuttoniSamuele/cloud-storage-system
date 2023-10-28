<script lang="ts">
  import { workingFolder } from "../stores/workingFolder";
  import ContentSection from "./ContentSection.svelte";
  import FolderContent from "./FolderContent.svelte";
  import FolderControls from "./FolderControls.svelte";
</script>

<main class="flex flex-col w-full h-full rounded-tl-lg bg-transparent">
  <ContentSection top>
    <FolderControls />
  </ContentSection>
  <ContentSection>
    {#if $workingFolder !== null}
      <FolderContent />
    {:else}
      <!-- TODO: -->
      <span class="text-zinc-700 dark:text-zinc-300">Please login</span>
    {/if}
  </ContentSection>
</main>
