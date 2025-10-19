<script lang="ts">
  import Navigation from "$lib/components/navigation.svelte";
  import "../app.css";

  let { children } = $props();
</script>

<main class="container w-screen h-screen">
  <section class="w-screen h-screen">
    <div class="w-screen h-screen flex flex-row">
        <Navigation />
        {@render children()}
    </div>
  </section>
</main>
