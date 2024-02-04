<script lang="ts">
  export let x: number;
  export let y: number;

  let navElem: HTMLElement | null = null;
  let windowWidth: number | null;
  let windowHeight: number | null;

  // Make sure that the context menu stays within the window
  $: if (navElem && windowWidth && windowHeight) {
    const width = navElem.clientWidth;
    const height = navElem.clientHeight;
    navElem.style.top = `${y + height > windowHeight ? y - height : y}px`;
    navElem.style.left = `${x + width > windowWidth ? x - width : x}px`;
  }
</script>

<svelte:window bind:innerWidth={windowWidth} bind:innerHeight={windowHeight} />

<nav
  bind:this={navElem}
  class="absolute z-50 border rounded-lg select-none bg-zinc-100 border-zinc-300 dark:bg-zinc-800 dark:border-zinc-600"
  on:contextmenu|preventDefault={() => {}}
>
  <ul>
    <slot />
  </ul>
</nav>
