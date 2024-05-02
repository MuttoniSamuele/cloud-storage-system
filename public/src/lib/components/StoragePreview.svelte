<script lang="ts">
  import API from "../logic/api";
  import { account } from "../stores/account";
  import { fileChange } from "../stores/fileChange";
  import StorageBar from "./StorageBar.svelte";

  $: personalFolderId = $account?.personalFolderId ?? null;
  $: trashFolderId = $account?.trashFolderId ?? null;
  $: totalSpace = ($account?.maxStorageMb ?? 0) * 1_000_000;

  async function getCloudSize(
    personalFolderId: number,
    trashFolderId: number,
    // This parameter is not used in the function, but it's necessary to
    // trigger the reactivity of the function when the file changes.
    _fileChange: string | null,
  ): Promise<number> {
    return (
      (await API.getFolderSize(personalFolderId)) +
      (await API.getFolderSize(trashFolderId))
    );
  }
</script>

{#if personalFolderId !== null && trashFolderId !== null}
  {#await getCloudSize(personalFolderId, trashFolderId, $fileChange.file)}
    <StorageBar loading {totalSpace} />
  {:then usedSpace}
    <StorageBar {usedSpace} {totalSpace} />
  {:catch}
    <StorageBar unknown {totalSpace} />
  {/await}
{:else}
  <StorageBar usedSpace={0} totalSpace={1} hideTotal />
{/if}
