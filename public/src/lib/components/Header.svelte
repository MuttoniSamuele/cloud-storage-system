<script lang="ts">
  import IconButton from "./IconButton.svelte";
  // import DarkModeButton from "./DarkModeButton.svelte";
  import ProfilePicture from "./ProfilePicture.svelte";
  // import SearchBar from "./SearchBar.svelte";
  import TextButton from "./TextButton.svelte";
  import { account } from "../stores/account";
  import { ModalState, modalState } from "../stores/modalState";
</script>

<header class="flex justify-between items-center w-full h-16 px-3">
  <a href="/" class="text-2xl uppercase text-zinc-800 dark:text-zinc-200">
    Cloud Storage
  </a>
  <!-- <SearchBar /> -->
  <div class="flex items-center">
    <!-- <DarkModeButton /> -->
    <!-- <IconButton icon="ri-settings-3-line" margin /> -->
    {#if $account === null}
      <TextButton
        text="Log in"
        marginX
        on:click={() => modalState.set(ModalState.Login)}
      />
    {:else}
      <ProfilePicture
        username={$account.username}
        on:click={() => modalState.set(ModalState.Cloud)}
      />
    {/if}
  </div>
</header>
