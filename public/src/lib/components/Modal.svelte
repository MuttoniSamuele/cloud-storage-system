<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import IconButton from "./IconButton.svelte";

  export let title: string;
  export let size: "sm" | "md" | "lg" | "fill" | "adapt";
  export let scrollable: boolean | null = null;
  export let noContainer = false;

  let dialog: HTMLDialogElement | null = null;
  $: dialog && dialog.showModal();

  const dispatch = createEventDispatcher<{ requestClose: void }>();

  function requestClose(): void {
    dispatch("requestClose");
  }
</script>

<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
<!-- svelte-ignore a11y-click-events-have-key-events -->
<dialog
  class="rounded-md bg-zinc-400 text-zinc-900 dark:bg-zinc-700 dark:text-zinc-200
    {size === 'sm'
    ? 'w-[26rem]'
    : size === 'md'
      ? 'w-[36rem]'
      : size === 'lg'
        ? 'w-[46rem]'
        : size === 'fill'
          ? 'w-4/5 h-4/5'
          : size === 'adapt'
            ? 'h-4/5'
            : ''}"
  bind:this={dialog}
  on:click|self={requestClose}
  on:keydown={(e) => {
    if (e.key === "Escape") {
      e.preventDefault();
      requestClose();
    }
  }}
>
  <div class="flex flex-col w-full h-full px-5 py-4">
    <div class="absolute top-3 right-3">
      <IconButton icon="ri-close-line" on:click={requestClose} />
    </div>
    <!-- TODO: Fix image stretch when title is too long in ImageFileModal -->
    <h2
      class="w-11/12 text-xl font-bold whitespace-nowrap overflow-hidden text-ellipsis"
    >
      {title}
    </h2>
    {#if noContainer}
      <slot />
    {:else}
      <div
        class="flex-1 mt-5 {scrollable === null
          ? ''
          : scrollable
            ? 'overflow-auto'
            : 'overflow-hidden'}"
      >
        <slot />
      </div>
    {/if}
  </div>
</dialog>
