<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { modalState, ModalState } from "../stores/modalState";
  import Modal from "./Modal.svelte";
  import TextButton from "./TextButton.svelte";
  import TextInput from "./TextInput.svelte";

  export let title: string;
  export let errorMessage: string | null = null;
  export let baseValue: string | null = null;
  export let autoSelect = false;

  let text = baseValue || "";

  const dispatch = createEventDispatcher<{ confirm: string }>();
</script>

<Modal
  {title}
  size="sm"
  on:requestClose={() => modalState.set(ModalState.Closed)}
>
  <TextInput
    wfull
    {baseValue}
    autofocus
    {autoSelect}
    on:input={({ detail }) => (text = detail)}
  />
  {#if errorMessage !== null}
    <div class="text-red-500 mt-2">
      {errorMessage}
    </div>
  {/if}
  <div class="flex justify-end w-full mt-5 mb-1">
    <TextButton text="Confirm" on:click={() => dispatch("confirm", text)} />
  </div>
</Modal>
