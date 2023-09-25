<script lang="ts">
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
    <button class="h-min mr-3">
      <i
        class="ri-settings-3-line ri-xl text-zinc-500 hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-zinc-100"
      />
    </button>
    <ProfilePicture />
  </div>
</header>
