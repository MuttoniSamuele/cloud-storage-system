<script lang="ts">
  import FileRow from "./FileRow.svelte";
  import FileRowField from "./FileRowField.svelte";

  export let showOwners = false;
</script>

<div class="relative">
  <div
    class="sticky top-0 flex items-center pl-5 pr-[44px] py-2 select-none border-b bg-zinc-100 dark:bg-zinc-800 border-zinc-300 dark:border-zinc-600"
  >
    <FileRowField size="large" isHeader>Name</FileRowField>
    <FileRowField size="small" secondary isHeader fixedMargin>
      Last modified
    </FileRowField>
    {#if showOwners}
      <FileRowField size="medium" secondary isHeader>Owner</FileRowField>
    {/if}
  </div>
  <ul>
    <FileRow name="Test 1" lastModified={new Date("1/2/3")} owner="Test" />
    <FileRow name="Test 2" owner="Test" />
    <FileRow name="Test 3" owner="Test" />
    <FileRow name="Test 4" owner="Test" />
    <FileRow name="Test 5" owner="Test" />
    <FileRow name="Test 6" owner="Test" />
    <FileRow name="Folder" isFolder owner="Test" />
    <FileRow
      name="Veeeeeeeeeeeeeeeeeeerrrrrrrrrrrrryyyyyyyyyy looooooooooooong foooooooooooooooooooldeeeeeeeeeeeeeerrrrrrrrrr"
      isFolder
      owner="Test"
    />
  </ul>
</div>
