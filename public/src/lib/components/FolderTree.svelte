<script lang="ts">
  import FolderAccordion from "./FolderAccordion.svelte";

  export let level: number;
  export let collapsed = true;

  // TODO: Add transition

  // function accordion(
  //   node: HTMLElement,
  //   args?: { duration?: number }
  // ): TransitionConfig {
  //   const fullHeight = node.offsetHeight;
  //   node.style.overflow = "hidden";
  //   return {
  //     duration: args?.duration,
  //     css: (t) => `
  //       height: ${t * fullHeight}px;
  //     `,
  //   };
  // }

  // function accordion(
  //   node: HTMLElement,
  //   {
  //     collapsed,
  //     onStart,
  //     onEnd,
  //   }: { collapsed: boolean; onStart?: () => void; onEnd?: () => void }
  // ) {
  //   const initialHeight = node.offsetHeight;
  //   node.style.height = collapsed ? "0" : "auto";
  //   node.style.overflow = "hidden";
  //   return {
  //     update(collapsed: boolean) {
  //       node.addEventListener("animationstart", () => {
  //         console.log("start (inner)");
  //         onStart && onStart();
  //       });
  //       node.addEventListener("animationstart", () => {
  //         console.log("end (inner)");
  //         onEnd && onEnd();
  //       });
  //       const animation = node.animate(
  //         [
  //           {
  //             height: initialHeight + "px",
  //             overflow: "hidden",
  //           },
  //           {
  //             height: 0,
  //             overflow: "hidden",
  //           },
  //         ],
  //         { duration: 100, fill: "both" }
  //       );
  //       animation.pause();
  //       if (collapsed) {
  //         animation.play();
  //       } else {
  //         animation.reverse();
  //       }
  //     },
  //   };
  // }

  async function fetchSubfolders(): Promise<string[]> {
    return ["Sub folder", "Test"];
  }
</script>

{#if !collapsed}
  {#await fetchSubfolders() then folders}
    <div>
      {#each folders as folder}
        <FolderAccordion name={folder} icon="ri-folder-3-fill" {level} />
      {/each}
    </div>
  {/await}
{/if}
