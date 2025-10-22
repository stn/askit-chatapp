<script lang="ts">
  import Navigation from "$lib/components/navigation.svelte";
  import "../app.css";

  let { children } = $props();
</script>

<main class="container w-screen h-[100dvh]">
  <section class="w-screen h-[100dvh]">
    <div class="w-screen h-[100dvh] flex flex-row">
        <Navigation />
        {@render children()}
    </div>
  </section>
</main>
