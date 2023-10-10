<script lang="ts">
  import Content from "./lib/components/Content.svelte";
  import Header from "./lib/components/Header.svelte";
  import Sidebar from "./lib/components/Sidebar.svelte";
</script>

<div class="flex flex-col w-full h-full bg-zinc-50 dark:bg-zinc-850">
  <Header />
  <div class="flex flex-1">
    <Sidebar />
    <Content />
  </div>
</div>
