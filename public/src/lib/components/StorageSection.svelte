<script lang="ts">
  import API from "../logic/api";
  import { account } from "../stores/account";
  import CloudSection from "./CloudSection.svelte";
  import ComplexStorageBar from "./ComplexStorageBar.svelte";
  import Loader from "./Loader.svelte";

  $: totalSpace = ($account?.maxStorageMb ?? 0) * 1_000_000;

  async function getUsedSpaces(
    personalFolderId: number,
    trashFolderId: number,
  ): Promise<[number, [string, number][]]> {
    const totUsedSpace =
      (await API.getFolderSize(personalFolderId)) +
      (await API.getFolderSize(trashFolderId));
    const sizes = await Promise.all<[string, number]>(
      ["Text", "Image"].map(async (fileType): Promise<[string, number]> => {
        const size =
          (await API.getFolderSize(personalFolderId, fileType)) +
          (await API.getFolderSize(trashFolderId, fileType));
        switch (fileType) {
          case "Text":
            return ["Text files", size];
          case "Image":
            return ["Images", size];
        }
        // This should never happen
        return ["", 0];
      }),
    );
    sizes.push([
      "Other files",
      totUsedSpace - sizes.reduce((acc, [, size]) => acc + size, 0),
    ]);
    return [totUsedSpace, sizes];
  }
</script>

<CloudSection title="Storage">
  {#if $account !== null}
    {#await getUsedSpaces($account.personalFolderId, $account.trashFolderId)}
      <div class="w-full flex justify-center my-2">
        <Loader />
      </div>
    {:then [totUsedSpace, usedSpaces]}
      <div class="mt-3">
        <ComplexStorageBar {usedSpaces} {totUsedSpace} {totalSpace} />
      </div>
    {:catch}
      <p class="text-red-500">Failed to retrieve storage information.</p>
    {/await}
  {/if}
</CloudSection>
