<script lang="ts">
  import FileCell from "./FileCell.svelte";
</script>

<div
  class="grid gap-8 justify-center p-4"
  style="grid-template-columns: repeat(auto-fill, minmax(8rem, 1fr));"
>
  <FileCell name="Test 0" />
  <FileCell name="Test 1" showOwner />
  <FileCell name="Test 2" showOwner />
  <FileCell name="Test 3" />
  <FileCell name="Test 4" />
  <FileCell name="Test 5" />
  <FileCell name="Test 6" showOwner />
  <FileCell name="Test 7 " />
  <FileCell name="Test 8" />
  <FileCell name="Test 9" />
  <FileCell name="Folder" isFolder />
  <FileCell name="Very long folder" isFolder showOwner />
</div>
