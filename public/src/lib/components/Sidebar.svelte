<script lang="ts">
  import Path from "../logic/Path";
  import { account } from "../stores/account";
  import FolderAccordion from "./FolderAccordion.svelte";
  import StoragePreview from "./StoragePreview.svelte";

  $: isLoggedIn = $account !== null;
</script>

<aside
  class="flex flex-col justify-between w-72 h-full overflow-x-hidden overflow-y-auto pr-3"
>
  <nav>
    <FolderAccordion
      displayName="My Cloud"
      icon="ri-hard-drive-2-fill"
      path={new Path("MyCloud")}
      droppable={isLoggedIn}
    />
    <FolderAccordion
      displayName="Shared with me"
      icon="ri-group-fill"
      path={new Path("Shared")}
      droppable={isLoggedIn}
    />
    <FolderAccordion
      displayName="Recent"
      icon="ri-history-fill"
      path={new Path("Recent")}
      droppable={false}
    />
    <FolderAccordion
      displayName="Trash"
      icon="ri-delete-bin-fill"
      path={new Path("Trash")}
      droppable={false}
    />
  </nav>
  <div class="mb-8">
    <StoragePreview usedSpace={isLoggedIn ? 5.3 : 0} totalSpace={15} />
  </div>
</aside>
