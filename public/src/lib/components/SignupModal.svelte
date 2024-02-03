<script lang="ts">
  import API from "../logic/api";
  import { modalState, ModalState } from "../stores/modalState";
  import Modal from "./Modal.svelte";
  import TextButton from "./TextButton.svelte";
  import TextInput from "./TextInput.svelte";
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
  />
  <TextInput
    id="signup-email"
    type="email"
    label="Email"
    placeholder="Email"
    wfull
    marginY
  />
  <TextInput
    id="signup-password"
    type="password"
    label="Password"
    placeholder="Password"
    wfull
    marginY
  />
  <TextInput
    id="signup-repeat-password"
    type="password"
    label="Repeat password"
    placeholder="Repeat password"
    wfull
    marginY
  />
  <div class="flex flex-col items-center w-full mt-6 mb-3">
    <TextButton
      text="Sign up"
      wide
      on:click={() => {
        API.login("User", "password");
        modalState.set(ModalState.Closed);
      }}
    />
  </div>
</Modal>
