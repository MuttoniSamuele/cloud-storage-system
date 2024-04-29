<script lang="ts">
  import { modalState, ModalState } from "../stores/modalState";
  import Modal from "./Modal.svelte";
  import AboutSection from "./AboutSection.svelte";
  import ProfileSection from "./ProfileSection.svelte";
  import StorageSection from "./StorageSection.svelte";
  import SectionDivider from "./SectionDivider.svelte";
</script>

<Modal
  title="My Cloud"
  size="md"
  on:requestClose={() => modalState.set(ModalState.Closed)}
>
  <div class="w-full mb-3">
    <ProfileSection />
    <SectionDivider />
    <StorageSection />
    <SectionDivider />
    <AboutSection />
  </div>
</Modal>
