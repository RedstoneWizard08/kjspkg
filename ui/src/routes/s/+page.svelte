<script lang="ts">
    import { _ } from "svelte-i18n";
    import { afterNavigate, goto, replaceState } from "$app/navigation";
    import { base } from "$app/paths";
    import {
        currentSearchStore,
        packagesStore,
        userPreferencesStore,
        updatePackagesStore,
        emptySearchResults,
    } from "$lib/stores";
    import { vsprintf } from "sprintf-js";
    import type { LoadingState, SortMode } from "$lib/types";
    import IconBlank from "$components/icons/IconBlank.svelte";
    import { onMount } from "svelte";
    import { page } from "$app/stores";
    import { guessSortMode } from "$lib/utils";
    import { contextMenu, type ContextMenuItem } from "$lib/contextMenu";
    import PackageList from "$components/ui/PackageList.svelte";
    import TablerIconCheck from "$components/icons/TablerIconCheck.svelte";
    import { siteConfig } from "$lib/config";
    import Icon from "@iconify/svelte";
    import { searchPackages } from "$api";

    let currentPage = $state(0);
    let perPage = $state(30);

    let loadingState: LoadingState = $state(
        $packagesStore.pagination.results == 0 ? "loading" : "ready",
    );

    const showDetails = $derived(($page.url.searchParams.get("showDetails") ?? "false") == "true");

    onMount(async () => {
        loadingState = (await updatePackagesStore()) ? "ready" : "failed";
        $userPreferencesStore.sortBy = guessSortMode($page.url.searchParams.get("sort") ?? "");

        if (
            $currentSearchStore == "" &&
            $page.url.searchParams.has("q") &&
            $page.url.searchParams.get("q") != ""
        ) {
            $currentSearchStore = $page.url.searchParams.get("q")!;
        }
    });

    $effect(() => {
        searchPackages($currentSearchStore, currentPage, perPage).then(
            (v) => ($packagesStore = v ?? emptySearchResults),
        );
    });

    const updateQuery = async () => {
        if ($currentSearchStore != "") $page.url.searchParams.set("q", $currentSearchStore);
        else $page.url.searchParams.delete("q");

        replaceState($page.url, $page.state);
    };

    afterNavigate((nav) => {
        if (nav.to?.route.id == "/s") {
            updateQuery();
        }
    });
</script>

<svelte:head>
    <title
        >{$currentSearchStore || $_(`search.title.${siteConfig.type}`)} - {siteConfig.siteName}</title
    >
</svelte:head>

<div class="mb-2 flex flex-wrap gap-2">
    {#if $currentSearchStore}
        <button
            class="variant-soft-secondary btn hover:variant-filled-primary w-fit"
            onclick={() => ($currentSearchStore = "")}
        >
            <Icon icon="tabler:clear-all" height="24" class="mr-2" />
            {$_("search.clear_filters")}
        </button>
    {/if}

    <button
        class="variant-soft-secondary btn hover:variant-filled-primary w-fit"
        onclick={() => ($userPreferencesStore.compact = !$userPreferencesStore.compact)}
    >
        <Icon icon="tabler:layout-dashboard" height="24" class="mr-2" />

        <span class="md:inline">
            {$userPreferencesStore.compact
                ? $_("search.use_view.list")
                : $_("search.use_view.compact")}
        </span>
    </button>
</div>

<div
    class="border-surface-600 bg-surface-900 rounded-bl-container-token rounded-br-container-token sticky top-[-1px] z-10 items-center justify-between p-2 backdrop-blur md:flex"
>
    <h1 class="h3">
        {#if !$currentSearchStore}
            {@html vsprintf($_("search.found_plural"), [$packagesStore.pagination.total])}
            {$_(`search.plural.${siteConfig.type}`)}
        {:else}
            {@html vsprintf(
                $packagesStore.pagination.total == 1
                    ? $_("search.found_singular")
                    : $_("search.found_plural"),
                [$packagesStore.pagination.total],
            )}

            <a href="{base}/s" class="anchor no-underline">
                {$packagesStore.pagination.total == 1
                    ? $_(`search.singular.${siteConfig.type}`)
                    : $_(`search.plural.${siteConfig.type}`)}
            </a>

            {#if $currentSearchStore != ""}
                {$_("search.matching")}
                <button
                    class="hover:variant-filled-error transition-all hover:rounded hover:p-1 hover:px-2 hover:line-through"
                    onclick={() => ($currentSearchStore = "")}
                >
                    {$currentSearchStore}
                </button>
            {/if}
        {/if}
    </h1>

    <div class="flex flex-row flex-wrap items-center space-x-2">
        {#each new Array($packagesStore.pagination.pages) as _, page}
            <button
                class="btn variant-glass-primary hover:variant-ghost-primary btn-icon-sm text-center font-bold transition-all"
                class:!variant-filled-primary={currentPage == page}
                onclick={() => (currentPage = page)}>{page}</button
            >
        {/each}
    </div>

    <div class="flex flex-wrap space-x-2">
        <button
            class="anchor"
            use:contextMenu={{
                initiator: "left",
                items: [
                    ...["name", "downloads", "published", "updated"].map(
                        (name) =>
                            ({
                                type: "ITEM",
                                label: $_(`search.sort_type.${name}`),
                                icon:
                                    $userPreferencesStore.sortBy == name
                                        ? TablerIconCheck
                                        : IconBlank,
                                action: () => ($userPreferencesStore.sortBy = name as SortMode),
                            }) as ContextMenuItem,
                    ),
                    { type: "SEPARATOR" },
                    {
                        type: "ITEM",
                        label: $_(`search.show_details`),
                        icon: showDetails ? TablerIconCheck : IconBlank,
                        action: () => {
                            if (showDetails) $page.url.searchParams.delete("showDetails");
                            else $page.url.searchParams.set("showDetails", "true");
                            goto(`?${$page.url.searchParams.toString()}`);
                        },
                    } as ContextMenuItem,
                ],
            }}
        >
            {$userPreferencesStore.sortBy != ""
                ? `${$_("search.sorted_by")} ${$_(`search.sorted_by.${$userPreferencesStore.sortBy}`)}`
                : $_("search.unsorted")}
        </button>
    </div>
</div>

{#if loadingState == "loading"}
    <dl
        class="grid grid-cols-1 gap-2"
        class:lg:grid-cols-2={!$userPreferencesStore.compact}
        class:md:grid-cols-2={$userPreferencesStore.compact}
        class:lg:grid-cols-3={$userPreferencesStore.compact}
    >
        {#each Array(5) as _}
            <div class="placeholder rounded-container-token h-24 w-full animate-pulse"></div>
        {/each}
    </dl>
{:else if loadingState == "ready" && $packagesStore.pagination.total == 0}
    <p class="text-center opacity-50">{$_(`errors.none_published.${siteConfig.type}`)}</p>
{:else if loadingState == "ready"}
    <dl
        class="grid grid-cols-1 gap-2"
        class:lg:grid-cols-2={!$userPreferencesStore.compact}
        class:md:grid-cols-2={$userPreferencesStore.compact}
        class:lg:grid-cols-3={$userPreferencesStore.compact}
    >
        <PackageList
            {showDetails}
            compact={$userPreferencesStore.compact}
            packages={$packagesStore}
        />
    </dl>
{:else if loadingState == "failed"}
    <!-- <p class="p-2">No projects found!</p> -->
{/if}
