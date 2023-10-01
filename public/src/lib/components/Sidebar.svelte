<script lang="ts">
  import FolderAccordion from "./FolderAccordion.svelte";
</script>

<aside class="w-72 h-full overflow-x-hidden overflow-y-auto">
  <nav>
    <FolderAccordion name="My Cloud" icon="ri-hard-drive-2-fill" />
    <FolderAccordion name="Shared with me" icon="ri-group-fill" />
    <FolderAccordion name="Recent" icon="ri-history-fill" droppable={false} />
    <FolderAccordion name="Trash" icon="ri-delete-bin-fill" droppable={false} />
  </nav>
</aside>
