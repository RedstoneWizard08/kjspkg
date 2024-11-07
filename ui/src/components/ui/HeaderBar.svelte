<script lang="ts">
    import { _ } from "svelte-i18n";
    import { AppBar } from "@skeletonlabs/skeleton";
    import { currentScrollPosition, currentSearchStore } from "$lib/stores";
    import { base } from "$app/paths";
    import { page } from "$app/stores";
    import { fly } from "svelte/transition";
    import IconAuth from "$components/auth/IconAuth.svelte";
    import { onMount } from "svelte";
    import { goto, replaceState } from "$app/navigation";
    import TablerIcon from "$components/icons/TablerIcon.svelte";

    let inputElement: HTMLInputElement = $state(null!);
    let active = $state(false);

    onMount(() => {
        $currentSearchStore = $page.route.id == "/s" ? ($page.url.searchParams.get("q") ?? "") : "";
    });

    const updateQuery = async () => {
        if ($page.route.id != "/s") {
            await goto("/s", { keepFocus: true });
        }

        if ($currentSearchStore != "") $page.url.searchParams.set("q", $currentSearchStore);
        else $page.url.searchParams.delete("q");

        replaceState($page.url, $page.state);
    };
</script>

<AppBar
    gridColumns="grid-cols-[auto_1fr_auto]"
    slotDefault="place-self-center !w-full"
    slotTrail="place-self-end"
    class="vt-none transition-colors"
    background={$currentScrollPosition.y > 16 ? "bg-surface-800/75" : "bg-transparent"}
>
    {#snippet lead()}
        <a class="flex items-center gap-2" href="{base}/">
            <img src="/kjspkg.png" alt="logo" class="rounded-token aspect-square w-8 min-w-8" />
            <span class="hidden lg:inline">KJSPKG</span>
        </a>
    {/snippet}

    {#snippet trail()}
        <IconAuth />
    {/snippet}

    <div class="flex flex-row items-center justify-start">
        {#if !active}
            <a
                href="/s"
                class="btn btn-primary transition-duration-300 variant-soft-secondary hover:variant-filled-secondary mr-4 flex flex-row items-center justify-center text-center transition-all"
                transition:fly={{ x: 10, duration: 100 }}>Browse</a
            >
        {/if}

        <div
            class="input-group input-group-divider w-full grid-cols-[1fr] transition-all lg:grid-cols-[auto_1fr]"
            transition:fly={{ y: -40 }}
        >
            <a href="/s" class="text-surface-400 hidden lg:inline">
                <TablerIcon name="search" class="hidden lg:block" />
            </a>

            <input
                type="search"
                class="w-full transition-all"
                placeholder={$_("search.placeholder")}
                bind:this={inputElement}
                bind:value={$currentSearchStore}
                onfocus={() => (active = true)}
                onblur={() => (active = false)}
                oninput={updateQuery}
                onchange={updateQuery}
            />
        </div>
    </div>
</AppBar>
