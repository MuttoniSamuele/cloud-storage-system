<script lang="ts">
  import ProfilePicture from "./ProfilePicture.svelte";
  import SearchBar from "./SearchBar.svelte";
  import { RiSettings3Line } from "svelte-remixicon";
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
      <RiSettings3Line
        class="w-7 text-zinc-500 hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-zinc-100"
        size="auto"
      />
    </button>
    <ProfilePicture />
  </div>
</header>
