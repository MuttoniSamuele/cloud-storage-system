<script lang="ts">
  import Path from "../logic/Path";
  import { account } from "../stores/account";
  import FolderAccordion from "./FolderAccordion.svelte";
  import OverflowYAuto from "./OverflowYAuto.svelte";
  import StoragePreview from "./StoragePreview.svelte";

  $: isLoggedIn = $account !== null;
</script>

<aside class="flex flex-col justify-between w-72 overflow-x-hidden">
  <OverflowYAuto>
    <!-- This overflow-y-auto is useless but it doesn't work without it -->
    <div class="overflow-y-auto">
      <nav class="pr-3">
        <FolderAccordion
          displayName="My Cloud"
          icon="ri-hard-drive-2-fill"
          path={new Path("MyCloud")}
          droppable={isLoggedIn}
        />
        <!-- <FolderAccordion
          displayName="Shared with me"
          icon="ri-group-fill"
          path={new Path("Shared")}
          droppable={false}
        /> -->
        <FolderAccordion
          displayName="Starred"
          icon="ri-star-fill"
          path={new Path("Starred")}
          droppable={false}
        />
        <FolderAccordion
          displayName="Trash"
          icon="ri-delete-bin-fill"
          path={new Path("Trash")}
          droppable={false}
        />
      </nav>
    </div>
  </OverflowYAuto>
  <div class="my-8">
    <StoragePreview usedSpace={isLoggedIn ? 5.3 : 0} totalSpace={15} />
  </div>
</aside>
