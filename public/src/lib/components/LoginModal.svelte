<script lang="ts">
  import API from "../logic/api";
  import { modalState, ModalState } from "../stores/modalState";
  import Modal from "./Modal.svelte";
  import TextButton from "./TextButton.svelte";
  import TextInput from "./TextInput.svelte";
</script>

<Modal
  title="Login"
  size="sm"
  on:requestClose={() => modalState.set(ModalState.Closed)}
>
  <TextInput
    id="login-email"
    type="email"
    label="Email"
    placeholder="Email"
    wfull
    marginY
  />
  <TextInput
    id="login-password"
    type="password"
    label="Password"
    placeholder="Password"
    wfull
    marginY
  />
  <div class="flex flex-col items-center w-full mt-6 mb-1">
    <TextButton
      text="Login"
      wide
      on:click={() => {
        API.login("User", "password");
        modalState.set(ModalState.Closed);
      }}
    />
    <div class="mt-5 mb-2 text-zinc-800 dark:text-zinc-200">
      Don't have an account?
      <TextButton text="Sign up" isLink />
    </div>
  </div>
</Modal>
