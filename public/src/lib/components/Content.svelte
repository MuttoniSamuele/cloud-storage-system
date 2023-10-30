<script lang="ts">
  import { workingFolder } from "../stores/workingFolder";
  import ContentSection from "./ContentSection.svelte";
  import FolderContent from "./FolderContent.svelte";
  import FolderControls from "./FolderControls.svelte";
  import NotLoggedIn from "./NotLoggedIn.svelte";
</script>

<main class="flex flex-col w-full h-full rounded-tl-lg bg-transparent">
  <ContentSection top>
    <FolderControls />
  </ContentSection>
  <ContentSection>
    {#if $workingFolder !== null}
      <FolderContent />
    {:else}
      <NotLoggedIn />
    {/if}
  </ContentSection>
</main>
