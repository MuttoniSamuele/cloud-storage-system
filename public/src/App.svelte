<script lang="ts">
  import Content from "./lib/components/Content.svelte";
  import Header from "./lib/components/Header.svelte";
  import ModalHandler from "./lib/components/ModalHandler.svelte";
  import Sidebar from "./lib/components/Sidebar.svelte";
  import { theme } from "./lib/stores/theme";

  // Little hack to always apply the dark theme
  theme.toggle();
</script>

<div class="flex flex-col w-full h-full bg-zinc-50 dark:bg-zinc-850">
  <Header />
  <div class="flex flex-1">
    <Sidebar />
    <Content />
  </div>
</div>
<ModalHandler />
