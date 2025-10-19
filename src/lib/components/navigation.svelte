<script lang="ts">
    import IconCat from "@lucide/svelte/icons/cat";
    import IconSettings from "@lucide/svelte/icons/settings";
</script>

<div class="flex-none flex flex-col border-r-[1px] border-surface-200-800">
    <!-- Header -->
    <header class="p-4">
        <a href="/" class="hover:opacity-70 transition-opacity cursor-pointer">
            <IconCat />
        </a>
    </header>
    <!-- Footer -->
    <footer class="p-4 mb-2">
        <a href="/settings" class="hover:opacity-70 transition-opacity cursor-pointer">
            <IconSettings />
        </a>
    </footer>
</div>
