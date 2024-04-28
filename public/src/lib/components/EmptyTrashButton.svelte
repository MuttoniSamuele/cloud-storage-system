<script lang="ts">
  import API from "../logic/api";
  import { account } from "../stores/account";
  import { fileChange } from "../stores/fileChange";
  import { pathsHistory } from "../stores/pathsHistory";
  import TextButton from "./TextButton.svelte";
</script>

{#if $account !== null}
  <TextButton
    icon="ri-upload-line"
    text="Empty trash"
    marginX
    on:click={async () => {
      await API.deleteFile($account.trashFolderId, true, true);
      fileChange.setFile("");
      pathsHistory.refresh();
    }}
  />
{/if}
