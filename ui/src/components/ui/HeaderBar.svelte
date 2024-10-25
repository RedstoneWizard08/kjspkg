<script lang="ts">
    import { _, locales } from "svelte-i18n";
    import { AppBar } from "@skeletonlabs/skeleton";
    import { currentScrollPosition, userPreferencesStore, currentSearchStore } from "$lib/stores";
    import { base } from "$app/paths";
    import { page } from "$app/stores";
    import { fly } from "svelte/transition";
    import { IconSearch, IconCheck, IconWorld, IconColorSwatch } from "@tabler/icons-svelte";
    import { contextMenu, type ContextMenuItem } from "$lib/contextMenu";
    import IconBlank from "$components/icons/IconBlank.svelte";
    import IconAuth from "$components/auth/IconAuth.svelte";
    import { onMount } from "svelte";
    import { replaceState } from "$app/navigation";

    let inputElement: HTMLInputElement = $state(null!);

    onMount(() => {
        $currentSearchStore = $page.route.id == "/s" ? ($page.url.searchParams.get("q") ?? "") : "";
    });

    const langContextMenu = $derived({
        initiator: "left" as const,
        items: [
            { type: "SEPARATOR", header: $_("menu.language") },

            ...$locales.map((lang) => ({
                type: "ITEM",
                label: lang,
                icon: $userPreferencesStore.locale == lang ? IconCheck : IconBlank,
                action: () => {
                    $userPreferencesStore.locale = lang;
                },
            })),
        ] as ContextMenuItem[],
    });

    const themeContextMenu = $derived({
        initiator: "left" as const,
        items: [
            { type: "SEPARATOR", header: $_("menu.theme") },

            ...[
                { label: "KJSPKG", name: "kjspkg" },
                { label: "(lighter)", name: "kjspkg-lighter" },
                { label: "(G_cat)", name: "kjspkg-gcat" },
                {},
                { label: "Wintry", name: "wintry" },
                { label: "Crimson", name: "crimson" },
                { label: "Serenity", name: "serenity" },
            ].map(({ label, name }) => {
                if (!label || !name) return { type: "SEPARATOR" };

                return {
                    type: "ITEM" as const,
                    label,
                    icon: $userPreferencesStore.theme == name ? IconCheck : IconBlank,
                    action: () => {
                        document.documentElement.classList.add("color-animated");
                        $userPreferencesStore.theme = name;
                        document.body.dataset.theme = $userPreferencesStore.theme ?? "kjspkg";
                    },
                };
            }),
        ] as ContextMenuItem[],
    });
</script>

<AppBar
    gridColumns="lg:grid-cols-3 grid-cols-[auto_1fr_auto]"
    slotDefault="place-self-center"
    slotTrail="place-self-end"
    class="vt-none transition-colors"
    background={$currentScrollPosition.y > 16 ? "bg-surface-800/75" : "bg-transparent"}
>
    <svelte:fragment slot="lead">
        <a class="flex items-center gap-2" href="{base}/">
            <img src="/kjspkg.png" alt="logo" class="aspect-square w-8 min-w-8 rounded-token" />
            <span class="hidden lg:inline">KJSPKG Lookup</span>
        </a>
    </svelte:fragment>

    {#if !$page.route.id || $page.route.id != "/stats"}
        <div
            class="input-group input-group-divider w-full grid-cols-[1fr] lg:w-fit lg:grid-cols-[auto_1fr]"
            transition:fly={{ y: -40 }}
        >
            <div class="hidden text-surface-400 lg:inline">
                <IconSearch class="hidden lg:block" />
            </div>

            <input
                type="search"
                placeholder={$_("search.placeholder")}
                bind:this={inputElement}
                bind:value={$currentSearchStore}
                onchange={() => {
                    if ($currentSearchStore != "")
                        $page.url.searchParams.set("q", $currentSearchStore);
                    else $page.url.searchParams.delete("q");
                    replaceState($page.url, $page.state);
                }}
            />
        </div>
    {/if}

    <svelte:fragment slot="trail">
        <span>
            <button class="btn-icon hover:variant-soft-primary" use:contextMenu={langContextMenu}>
                <IconWorld />
            </button>

            <button class="btn-icon hover:variant-soft-primary" use:contextMenu={themeContextMenu}>
                <IconColorSwatch />
            </button>
        </span>

        <IconAuth />
    </svelte:fragment>
</AppBar>
