<script lang="ts">
  import { account } from "../stores/account";
  import { ModalState, modalState } from "../stores/modalState";
  import TextButton from "./TextButton.svelte";
</script>

<TextButton
  icon="ri-folder-add-line"
  text="New folder"
  marginX
  on:click={() => $account !== null && modalState.set(ModalState.NewFolder)}
/>
