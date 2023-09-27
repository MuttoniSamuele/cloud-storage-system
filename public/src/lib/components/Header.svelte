<script lang="ts">
  import IconButton from "./IconButton.svelte";
  import DarkModeButton from "./DarkModeButton.svelte";
  import ProfilePicture from "./ProfilePicture.svelte";
  import SearchBar from "./SearchBar.svelte";
</script>

<header
  class="flex justify-between items-center w-full h-16 px-3 bg-zinc-50 dark:bg-zinc-850"
>
  <a href="/" class="text-2xl uppercase text-zinc-800 dark:text-zinc-200">
    Cloud Storage
  </a>
  <SearchBar />
  <div class="flex items-center">
    <DarkModeButton />
    <IconButton icon="ri-settings-3-line" />
    <ProfilePicture />
  </div>
</header>
