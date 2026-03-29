<script lang="ts">
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";

  onMount(() => {
    goto("/keyboard");
  });
</script>

<div class="home">
  <p>Redirecionando...</p>
</div>
