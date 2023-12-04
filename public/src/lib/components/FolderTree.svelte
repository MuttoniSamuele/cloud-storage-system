<script lang="ts">
  import type Path from "../logic/Path";
  import API from "../logic/api";
  import FolderAccordion from "./FolderAccordion.svelte";

  export let path: Path;
  export let level: number;
  export let collapsed = true;

  // TODO: Add transition
</script>

{#if !collapsed}
  {#await API.getFiles(path, { foldersOnly: true }) then folders}
    <div>
      {#each folders as folder}
        <FolderAccordion
          displayName={folder.name}
          icon="ri-folder-3-fill"
          {level}
          droppable={folder.fileType === "NotEmpty"}
          path={(() => {
            // This is an IIFE that clones the path, adds a subfolder and returns it
            const p = path.clone();
            p.addSubFolder(folder.name);
            return p;
          })()}
        />
      {/each}
    </div>
  {/await}
{/if}
