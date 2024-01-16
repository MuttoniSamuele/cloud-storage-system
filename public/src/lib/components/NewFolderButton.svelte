<script lang="ts">
  import TextButton from "./TextButton.svelte";
  import { ModalState, modalState } from "../stores/modalState";
  import { account } from "../stores/account";
</script>

<TextButton
  icon="ri-folder-add-line"
  text="New folder"
  marginX
  on:click={() => $account !== null && modalState.set(ModalState.TextInput)}
/>
