<script lang="ts">
  import API from "../logic/api";
  import { modalState, ModalState } from "../stores/modalState";
  import Modal from "./Modal.svelte";
  import TextButton from "./TextButton.svelte";
  import TextInput from "./TextInput.svelte";

  let username = "";
  let email = "";
  let password = "";
  let repeatedPassword = "";
  let errorMessage: string | null = null;
  let isValidating = false;

  $: canSignup =
    [username, email, password, repeatedPassword].every((s) => s !== "") &&
    !isValidating;

  $: console.log(canSignup);

  async function handleSignupClick(): Promise<void> {
    isValidating = true;
    if (password !== repeatedPassword) {
      errorMessage = "The passwords don't match.";
      isValidating = false;
      return;
    }
    try {
      await API.signup(username, email, password);
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
  title="Sign up"
  size="sm"
  on:requestClose={() => modalState.set(ModalState.Closed)}
>
  <TextInput
    id="signup-username"
    label="Username"
    placeholder="Username"
    wfull
    marginY
    on:input={(e) => (username = e.detail)}
  />
  <TextInput
    id="signup-email"
    type="email"
    label="Email"
    placeholder="Email"
    wfull
    marginY
    on:input={(e) => (email = e.detail)}
  />
  <TextInput
    id="signup-password"
    type="password"
    label="Password"
    placeholder="Password"
    wfull
    marginY
    on:input={(e) => (password = e.detail)}
  />
  <TextInput
    id="signup-repeat-password"
    type="password"
    label="Repeat password"
    placeholder="Repeat password"
    wfull
    marginY
    on:input={(e) => (repeatedPassword = e.detail)}
  />
  {#if errorMessage !== null}
    <div class="text-red-500">
      {errorMessage}
    </div>
  {/if}
  <div class="flex flex-col items-center w-full mt-6 mb-3">
    <TextButton
      text="Sign up"
      wide
      on:click={handleSignupClick}
      disabled={!canSignup}
    />
  </div>
</Modal>
