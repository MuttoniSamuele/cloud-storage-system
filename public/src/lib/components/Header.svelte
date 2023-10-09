<script lang="ts">
  import IconButton from "./IconButton.svelte";
  import DarkModeButton from "./DarkModeButton.svelte";
  import ProfilePicture from "./ProfilePicture.svelte";
  import SearchBar from "./SearchBar.svelte";
  import TextButton from "./TextButton.svelte";
  import { account } from "../stores/account";
  import { workingFolder } from "../stores/workingFolder";
  import Path from "../logic/Path";

  async function login(username: string, password: string): Promise<void> {
    const options = {
      method: "POST",
      headers: new Headers({ "content-type": "application/json" }),
      body: JSON.stringify({ username: username, password: password }),
    };
    const res = await fetch("/dummy/login", options);
    if (res.ok) {
      account.login(username);
      workingFolder.change(new Path("MyCloud"));
    }
  }
</script>

<header class="flex justify-between items-center w-full h-16 px-3">
  <a href="/" class="text-2xl uppercase text-zinc-800 dark:text-zinc-200">
    Cloud Storage
  </a>
  <SearchBar />
  <div class="flex items-center">
    <DarkModeButton />
    <IconButton icon="ri-settings-3-line" margin />
    {#if $account === null}
      <TextButton text="Log in" on:click={() => login("User", "password")} />
    {:else}
      <ProfilePicture />
    {/if}
  </div>
</header>
