<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { fileTypeToIcon, type FileType } from "../logic/fileUtils";
  import IconButton from "./IconButton.svelte";
  import ProfilePicture from "./ProfilePicture.svelte";

  export let name: string;
  export let isFolder = false;
  export let fileType: FileType | null = null;
  export let owner: string | null = null;
  export let selected = false;

  const dispatch = createEventDispatcher<{ more: void }>();
</script>

<div
  class="relative flex flex-col w-32 h-32 p-2 rounded-sm
  {selected
    ? 'bg-zinc-300 dark:bg-zinc-600'
    : 'hover:bg-zinc-200 dark:hover:bg-zinc-700'}"
>
  <button
    class="absolute top-0 left-0 w-full h-full cursor-default"
    on:click
    on:dblclick
  ></button>
  <div class="flex justify-center items-center w-full flex-1">
    <i
      class="{isFolder
        ? 'ri-folder-3-fill text-indigo-400'
        : `${fileTypeToIcon(
            fileType || 'Unsupported',
          )} text-zinc-400 dark:text-zinc-200`}
        ri-4x -my-10"
    />
  </div>
  <div class="flex items-center w-full">
    <span
      class="inline-block w-full text-center select-none whitespace-nowrap overflow-ellipsis overflow-hidden text-zinc-900 dark:text-zinc-200"
    >
      {name}
    </span>
  </div>
  <div class="absolute right-2">
    <IconButton icon="ri-more-line" on:click={() => dispatch("more")} />
  </div>
  {#if owner !== null}
    <div
      class="absolute {isFolder ? 'right-5 bottom-11' : 'right-6 bottom-10'} "
    >
      <ProfilePicture username={owner} small />
    </div>
  {/if}
</div>
