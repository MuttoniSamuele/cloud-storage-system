<script lang="ts">
  export let username: string;
  export let small = false;
  export let big = false;
</script>

<button class="mx-2" title={username} on:click>
  <img
    class="rounded-full object-cover
    {small ? 'w-5 h-5' : big ? 'w-16 h-16' : 'w-8 h-8'}"
    src="default-pfp.png"
    alt="Profile"
  />
</button>
