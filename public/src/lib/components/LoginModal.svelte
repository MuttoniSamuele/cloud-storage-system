<script lang="ts">
  import API from "../logic/api";
  import { modalState, ModalState } from "../stores/modalState";
  import Modal from "./Modal.svelte";
  import TextButton from "./TextButton.svelte";
  import TextInput from "./TextInput.svelte";

  let email = "";
  let password = "";
  let errorMessage: string | null = null;
  let isValidating = false;

  $: canLogin = [email, password].every((s) => s !== "") && !isValidating;

  async function handleLoginClick(): Promise<void> {
    isValidating = true;
    try {
      await API.login(email, password);
    } catch (e) {
      isValidating = false;
      if (e instanceof API.ApiError) {
        errorMessage = e.message;
        return;
      }
      throw e;
    }
    modalState.set(ModalState.Closed);
  }
</script>

<Modal
  title="Log in"
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
    on:input={(e) => (email = e.detail)}
  />
  <TextInput
    id="login-password"
    type="password"
    label="Password"
    placeholder="Password"
    wfull
    marginY
    on:input={(e) => (password = e.detail)}
  />
  {#if errorMessage !== null}
    <div class="text-red-500">
      {errorMessage}
    </div>
  {/if}
  <div class="flex flex-col items-center w-full mt-6 mb-1">
    <TextButton
      text="Log in"
      wide
      on:click={handleLoginClick}
      disabled={!canLogin}
    />
    <div class="mt-5 mb-2 text-zinc-800 dark:text-zinc-200">
      Don't have an account?
      <TextButton
        text="Sign up"
        isLink
        on:click={() => modalState.set(ModalState.Signup)}
      />
    </div>
  </div>
</Modal>
