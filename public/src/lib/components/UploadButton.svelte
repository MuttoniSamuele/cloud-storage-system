<script lang="ts">
  import { account } from "../stores/account";
  import { ModalState, modalState } from "../stores/modalState";
  import TextButton from "./TextButton.svelte";
</script>

<TextButton
  icon="ri-upload-line"
  text="Upload"
  marginX
  on:click={() => $account !== null && modalState.set(ModalState.Upload)}
/>
