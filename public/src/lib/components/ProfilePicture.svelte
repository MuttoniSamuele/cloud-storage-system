<script lang="ts">
  export let username: string;
  export let small = false;
</script>

<button class="mx-2" title={username}>
  <img
    class="rounded-full object-cover
      {small ? 'w-5 h-5' : 'w-8 h-8'}"
    src="default-pfp.png"
    alt="Profile"
  />
</button>
