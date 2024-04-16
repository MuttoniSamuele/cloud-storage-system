<script lang="ts">
  import type Path from "../logic/Path";
  import API from "../logic/api";
  import { cmpFileNames } from "../logic/fileUtils";
  import FolderAccordion from "./FolderAccordion.svelte";

  export let path: Path;
  export let level: number;
  export let collapsed = true;
</script>

{#if !collapsed}
  {#await API.view(path.lastId, true) then { folders }}
    <div>
      {#each folders.sort(cmpFileNames) as folder}
        <FolderAccordion
          displayName={folder.name}
          icon="ri-folder-3-fill"
          {level}
          droppable
          path={(() => {
            // This is an IIFE that clones the path, adds a subfolder and returns it
            const p = path.clone();
            p.addSubFolder({ id: folder.id, name: folder.name });
            return p;
          })()}
        />
      {/each}
    </div>
  {/await}
{/if}
