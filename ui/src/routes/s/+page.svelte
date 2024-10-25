<script lang="ts">
    import { _ } from "svelte-i18n";
    import { goto, replaceState } from "$app/navigation";
    import { base } from "$app/paths";
    import {
        currentSearchStore,
        packagesStore,
        userPreferencesStore,
        filteredStore,
        updatePackagesStore,
    } from "$lib/stores";
    import { IconCheck, IconClearAll, IconLayoutDashboard } from "@tabler/icons-svelte";
    import { vsprintf } from "sprintf-js";
    import type { LoadingState, SortMode } from "$lib/types";
    import IconBlank from "$components/icons/IconBlank.svelte";
    import { onMount } from "svelte";
    import { page } from "$app/stores";
    import { guessSortMode } from "$lib/utils";
    import { contextMenu, type ContextMenuItem } from "$lib/contextMenu";
    import PackageList from "$components/ui/PackageList.svelte";

    let optionsHeader: HTMLDivElement = $state(null!);
    let loadingState: LoadingState = $state($packagesStore.length == 0 ? "loading" : "ready");
    let showDetails = $state(false);

    onMount(async () => {
        loadingState = (await updatePackagesStore()) ? "ready" : "failed";
        showDetails = ($page.url.searchParams.get("showDetails") ?? "false") == "true";
        $userPreferencesStore.sortBy = guessSortMode($page.url.searchParams.get("sort") ?? "");
        $currentSearchStore = $page.url.searchParams.get("q") ?? "";
    });

    const observer = new IntersectionObserver(
        ([e]) => {
            // e.target.classList.toggle('backdrop-brightness-75', e.intersectionRatio < 1);
            e.target.classList.toggle("border-b", e.intersectionRatio < 1);
        },
        { threshold: [1] },
    );

    $effect(() => {
        if ($currentSearchStore != "") $page.url.searchParams.set("q", $currentSearchStore);
        else $page.url.searchParams.delete("q");
        replaceState($page.url, $page.state);
    });

    $effect(() => {
        optionsHeader ? observer.observe(optionsHeader) : "";
    });

    let largeScreen = matchMedia("(min-width: 1024px)").matches;

    window.onresize = () => {
        largeScreen = matchMedia("(min-width: 1024px)").matches;
    };
</script>

<svelte:head>
    <title>{$currentSearchStore || "Search"} - KJSPKG</title>
</svelte:head>

<div class="mb-2 flex flex-wrap gap-2">
    {#if $currentSearchStore}
        <button
            class="variant-soft-secondary btn w-fit hover:variant-filled-primary"
            onclick={() => ($currentSearchStore = "")}
        >
            <IconClearAll class="mr-2" />
            {$_("search.clear_filters")}
        </button>
    {/if}

    <button
        class="variant-soft-secondary btn w-fit hover:variant-filled-primary"
        onclick={() => ($userPreferencesStore.compact = !$userPreferencesStore.compact)}
    >
        <IconLayoutDashboard class="mr-2" />

        <!-- <span class="inline md:hidden">
			{$userPreferencesStore.compact ? 'Show icons' : 'Hide icons'}
		</span> -->

        <span class="hidden md:inline">
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
            {$_("search.package_plural")}
        {:else}
            {@html vsprintf(
                $filteredStore.length == 1
                    ? $_("search.found_singular")
                    : $_("search.found_plural"),
                [$filteredStore.length],
            )}

            <a href="{base}/s" class="anchor no-underline">
                {$filteredStore.length == 1
                    ? $_("search.package_singular")
                    : $_("search.package_plural")}
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
                    ...["name", "author", "downloads", "views"].map(
                        (name) =>
                            ({
                                type: "ITEM",
                                label: $_(`search.sort_type.${name}`),
                                icon: $userPreferencesStore.sortBy == name ? IconCheck : IconBlank,
                                action: () => ($userPreferencesStore.sortBy = name as SortMode),
                            }) as ContextMenuItem,
                    ),
                    { type: "SEPARATOR" },
                    {
                        type: "ITEM",
                        label: $_(`search.show_details`),
                        icon: showDetails ? IconCheck : IconBlank,
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
                : "Unsorted"}
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
    <p class="text-center opacity-50">{$_("errors.no_packages_published")}</p>
{:else if loadingState == "ready"}
    <dl
        class="grid grid-cols-1 gap-2"
        class:lg:grid-cols-2={!$userPreferencesStore.compact}
        class:md:grid-cols-2={$userPreferencesStore.compact}
        class:lg:grid-cols-3={$userPreferencesStore.compact}
    >
        <PackageList {showDetails} compact={$userPreferencesStore.compact} />
    </dl>
{:else if loadingState == "failed"}
    <p>{$_("errors.something_went_wrong")}</p>
{/if}
