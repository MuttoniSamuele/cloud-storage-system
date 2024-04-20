<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import type { HTMLInputTypeAttribute } from "svelte/elements";

  /** It's recommended to specify an ID when using a label */
  export let label: string | null = null;
  export let id: string | null = label;
  export let type: HTMLInputTypeAttribute = "text";
  export let placeholder: string = "";
  export let wfull = false;
  export let marginX = false;
  export let marginY = false;
  export let baseValue: string | null = null;

  const dispatch = createEventDispatcher<{ input: string }>();
</script>

<div
  class="
    {wfull ? 'w-full' : ''}
    {marginX ? 'mx-2' : ''}
    {marginY ? 'my-2' : ''}"
>
  {#if label !== null}
    <label for={id} class="block text-zinc-800 dark:text-zinc-300 mb-1">
      {label}
    </label>
  {/if}
  <input
    {type}
    {id}
    {placeholder}
    value={baseValue}
    class="px-2 py-1 rounded border border-zinc-700 dark:border-zinc-400 bg-transparent placeholder-zinc-600 dark:placeholder-zinc-400 text-zinc-900 dark:text-zinc-200
      {wfull ? 'w-full' : ''}"
    on:input={(e) => dispatch("input", e.currentTarget.value)}
  />
</div>
