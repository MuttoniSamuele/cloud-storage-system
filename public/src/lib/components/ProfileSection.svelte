<script lang="ts">
  import API from "../logic/api";
  import { account } from "../stores/account";
  import CloudSection from "./CloudSection.svelte";
  import ProfilePicture from "./ProfilePicture.svelte";
  import TextButton from "./TextButton.svelte";
</script>

<CloudSection title="Profile">
  {#if $account}
    <div class="flex items-center">
      <div class="dark:text-zinc-400 text-sm">
        <div class="w-full my-1 flex items-end">
          <span class="w-20">Username</span>
          <span class="dark:text-zinc-100 text-base">
            {$account.username}
          </span>
        </div>
        <div class="w-full my-1 flex items-end">
          <span class="w-20">Email</span>
          <span class="dark:text-zinc-100 text-base">
            {$account.email}
          </span>
        </div>
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
        on:click={async () => await API.deleteMe()}
      />
    </div>
  {/if}
</CloudSection>
