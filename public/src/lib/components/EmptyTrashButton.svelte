<script lang="ts">
  import API from "../logic/api";
  import { formatBytes } from "../logic/fileUtils";
  import { account } from "../stores/account";
  import { fileChange } from "../stores/fileChange";
  import { pathsHistory } from "../stores/pathsHistory";
  import Loader from "./Loader.svelte";
  import TextButton from "./TextButton.svelte";

  async function getTrashSize(
    trashFolderId: number,
    // This parameter is not used in the function, but it's necessary to
    // trigger the reactivity of the function when the file changes.
    _fileChange: string | null,
  ): Promise<number> {
    return await API.getFolderSize(trashFolderId);
  }
</script>

{#if $account !== null}
  <TextButton
    icon="ri-delete-bin-line"
    text="Empty trash"
    marginX
    slot
    on:click={async () => {
      await API.deleteFile($account.trashFolderId, true, true);
      fileChange.setFile("");
      pathsHistory.refresh();
    }}
  >
    {#await getTrashSize($account.trashFolderId, $fileChange.file)}
      (<Loader small inline />)
    {:then size}
      ({formatBytes(size)})
    {/await}
  </TextButton>
{/if}
