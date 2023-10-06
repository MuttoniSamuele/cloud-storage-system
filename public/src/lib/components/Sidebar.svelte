<script lang="ts">
  import FolderAccordion from "./FolderAccordion.svelte";
  import StoragePreview from "./StoragePreview.svelte";
</script>

<aside
  class="flex flex-col justify-between w-72 h-full overflow-x-hidden overflow-y-auto"
>
  <nav>
    <FolderAccordion
      displayName="My Cloud"
      icon="ri-hard-drive-2-fill"
      path="/MyCloud"
    />
    <FolderAccordion
      displayName="Shared with me"
      icon="ri-group-fill"
      path="/MyCloud"
    />
    <FolderAccordion
      displayName="Recent"
      icon="ri-history-fill"
      droppable={false}
    />
    <FolderAccordion
      displayName="Trash"
      icon="ri-delete-bin-fill"
      droppable={false}
    />
  </nav>
  <div class="mb-8">
    <StoragePreview usedSpace={5.3} totalSpace={15} />
  </div>
</aside>
