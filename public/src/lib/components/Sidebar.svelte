<script lang="ts">
  import FolderAccordion from "./FolderAccordion.svelte";
</script>

<aside class="w-72 h-full overflow-x-hidden overflow-y-auto">
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
</aside>
