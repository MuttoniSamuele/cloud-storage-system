<script lang="ts">
  import FileRow from "./FileRow.svelte";
</script>

<ul>
  <FileRow name="Test 1" lastModified={new Date("1/2/3")} />
  <FileRow name="Test 2" />
  <FileRow name="Test 3" />
  <FileRow name="Test 4" owner="Example Owner" />
  <FileRow name="Test 5" />
  <FileRow name="Test 6" />
  <FileRow name="Folder" isFolder />
  <FileRow
    name="Veeeeeeeeeeeeeeeeeeerrrrrrrrrrrrryyyyyyyyyy looooooooooooong foooooooooooooooooooldeeeeeeeeeeeeeerrrrrrrrrr"
    isFolder
  />
</ul>
