<script lang="ts">
    import { page } from "$app/stores";
    import { slide } from "svelte/transition";
    import PackageList from "./PackageList.svelte";
    import { currentScrollPosition, filteredStore, updateFilteredPackages, updatePackagesStore } from "$lib/stores";
    import { onMount } from "svelte";

    const sidebarHidden = $derived($page.route.id == "/" || $page.route.id == "/s");

    onMount(async () => {
        if (!sidebarHidden) {
            await updatePackagesStore();
            updateFilteredPackages();
        }
    });
</script>

{#if !sidebarHidden}
    <div
        class="border-surface-800 flex h-full w-96 flex-col gap-2 overflow-y-scroll p-2 {$currentScrollPosition.y >
        16
            ? 'border-r'
            : 'border-none'}"
        transition:slide={{ axis: "x" }}
    >
        {#if $filteredStore.length > 0 && $page.route.id != "/stats"}
            <h3 class="h4 mt-4 text-center">Search results</h3>
            <PackageList />
        {/if}
    </div>
{/if}
