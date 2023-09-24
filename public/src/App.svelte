<script lang="ts">
  import Content from "./lib/components/Content.svelte";
  import Header from "./lib/components/Header.svelte";
  import Sidebar from "./lib/components/Sidebar.svelte";
</script>

<main class="flex flex-col w-full h-full">
  <Header />
  <div class="flex flex-1">
    <Sidebar />
    <Content />
  </div>
</main>
