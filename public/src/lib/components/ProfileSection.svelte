<script lang="ts">
  import API from "../logic/api";
  import { account } from "../stores/account";
  import { ModalState, modalState } from "../stores/modalState";
  import CloudSection from "./CloudSection.svelte";
  import LabelledText from "./LabelledText.svelte";
  import ProfilePicture from "./ProfilePicture.svelte";
  import TextButton from "./TextButton.svelte";
</script>

<CloudSection title="Profile">
  {#if $account}
    <div class="flex items-center">
      <div>
        <LabelledText label="Username" text={$account.username} />
        <LabelledText label="Email" text={$account.email} />
      </div>
      <div class="ml-auto">
        <ProfilePicture username={$account.username} big />
      </div>
    </div>

    <div class="flex mt-3">
      <div class="mr-4">
        <TextButton
          text="Logout"
          dangerous
          on:click={async () => await API.logout()}
        />
      </div>
      <TextButton
        text="Delete account"
        dangerous
        on:click={() => modalState.set(ModalState.Confirmation)}
      />
    </div>
  {/if}
</CloudSection>
