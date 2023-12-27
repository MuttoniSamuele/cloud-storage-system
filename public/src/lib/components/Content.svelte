<script lang="ts">
  import { account } from "../stores/account";
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
    {#if $account === null}
      <NotLoggedIn />
    {:else}
      <FolderContent />
    {/if}
  </ContentSection>
</main>
