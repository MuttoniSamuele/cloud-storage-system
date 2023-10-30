<script lang="ts">
  import TextButton from "./TextButton.svelte";
  import * as API from "../logic/api";
</script>

<div class="flex justify-center items-center w-full h-full">
  <div class="flex flex-col items-center">
    <i
      class="ri-account-box-line ri-4x -my-0 text-zinc-400 dark:text-zinc-500"
    />
    <span class="mb-5 text-lg text-zinc-700 dark:text-zinc-200">
      You must have an account in order to use this service.
    </span>
    <TextButton
      text="Sign in"
      marginY
      wide
      on:click={() => API.login("User", "password")}
    />
    <TextButton
      text="Log in"
      marginY
      wide
      on:click={() => API.login("User", "password")}
    />
  </div>
</div>
