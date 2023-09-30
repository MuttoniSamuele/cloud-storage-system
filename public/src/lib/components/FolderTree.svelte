<script lang="ts">
  import FolderAccordion from "./FolderAccordion.svelte";

  export let level: number;

  async function fetchSubfolders(): Promise<string[]> {
    return ["Sub folder", "Test"];
  }
</script>

<div>
  {#await fetchSubfolders() then folders}
    {#each folders as folder}
      <FolderAccordion name={folder} icon="ri-folder-3-fill" {level} />
    {/each}
  {/await}
</div>
