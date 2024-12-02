<script lang="ts">
    import { _ } from "svelte-i18n";
    import { afterNavigate, goto, replaceState } from "$app/navigation";
    import { base } from "$app/paths";
    import {
        currentSearchStore,
        packagesStore,
        userPreferencesStore,
        filteredStore,
        updatePackagesStore,
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

    let optionsHeader: HTMLDivElement = $state(null!);
    let loadingState: LoadingState = $state($packagesStore.length == 0 ? "loading" : "ready");

    const showDetails = $derived(($page.url.searchParams.get("showDetails") ?? "false") == "true");

    onMount(async () => {
        loadingState = (await updatePackagesStore()) ? "ready" : "failed";
        $userPreferencesStore.sortBy = guessSortMode($page.url.searchParams.get("sort") ?? "");

        let largeScreen = matchMedia("(min-width: 1024px)").matches;

        window.onresize = () => {
            largeScreen = matchMedia("(min-width: 1024px)").matches;
        };

        if (
            $currentSearchStore == "" &&
            $page.url.searchParams.has("q") &&
            $page.url.searchParams.get("q") != ""
        ) {
            $currentSearchStore = $page.url.searchParams.get("q")!;
        }
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
            class="variant-soft-secondary btn w-fit hover:variant-filled-primary"
            onclick={() => ($currentSearchStore = "")}
        >
            <Icon icon="tabler:clear-all" height="24" class="mr-2" />
            {$_("search.clear_filters")}
        </button>
    {/if}

    <button
        class="variant-soft-secondary btn w-fit hover:variant-filled-primary"
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
    class="sticky top-[-1px] z-10 justify-between border-surface-600 bg-surface-900 p-2 backdrop-blur rounded-bl-container-token rounded-br-container-token md:flex"
    bind:this={optionsHeader}
>
    <h1 class="h3">
        {#if !$currentSearchStore}
            {@html vsprintf($_("search.found_plural"), [$filteredStore.length])}
            {$_(`search.plural.${siteConfig.type}`)}
        {:else}
            {@html vsprintf(
                $filteredStore.length == 1
                    ? $_("search.found_singular")
                    : $_("search.found_plural"),
                [$filteredStore.length],
            )}

            <a href="{base}/s" class="anchor no-underline">
                {$filteredStore.length == 1
                    ? $_(`search.singular.${siteConfig.type}`)
                    : $_(`search.plural.${siteConfig.type}`)}
            </a>

            {#if $currentSearchStore != ""}
                {$_("search.matching")}
                <button
                    class="transition-all hover:variant-filled-error hover:rounded hover:p-1 hover:px-2 hover:line-through"
                    onclick={() => ($currentSearchStore = "")}
                >
                    {$currentSearchStore}
                </button>
            {/if}
        {/if}
    </h1>

    <div class="flex flex-wrap space-x-2">
        <button
            class="anchor"
            use:contextMenu={{
                initiator: "left",
                items: [
                    ...["name", "downloads", "views", "published", "updated"].map(
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
            <div class="placeholder h-24 w-full animate-pulse rounded-container-token"></div>
        {/each}
    </dl>
{:else if loadingState == "ready" && $filteredStore.length == 0}
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
            packages={$filteredStore}
        />
    </dl>
{:else if loadingState == "failed"}
    <p>{$_("errors.something_went_wrong")}</p>
{/if}
