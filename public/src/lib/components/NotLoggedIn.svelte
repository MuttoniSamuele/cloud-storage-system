<script lang="ts">
  import TextButton from "./TextButton.svelte";
  import API from "../logic/api";
  import { ModalState, modalState } from "../stores/modalState";
</script>

<div class="flex justify-center items-center w-full h-full">
  <div class="flex flex-col items-center mb-14">
    <i class="ri-account-box-line ri-4x text-zinc-400 dark:text-zinc-500" />
    <span class="mb-5 text-lg text-zinc-700 dark:text-zinc-200">
      You must have an account in order to use this service.
    </span>
    <TextButton
      text="Log in"
      marginY
      wide
      on:click={() => modalState.set(ModalState.Login)}
    />
    <TextButton
      text="Sign up"
      marginY
      wide
      on:click={() => API.login("User", "password")}
    />
  </div>
</div>
