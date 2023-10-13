<script lang="ts">
  import Breadcrumb from "./Breadcrumb.svelte";
  import IconButton from "./IconButton.svelte";
  import Navigation from "./Navigation.svelte";
</script>

<!--
- back, forward
- refresh
- breadcrumb
- upload
- create folder
- switch grid/row
- filter
-->
<section class="flex">
  <Navigation />
  <IconButton icon="ri-restart-line ml-1 mr-2" small />
  <Breadcrumb />
</section>
