<script lang="ts">
    import { _ } from "svelte-i18n";
    import { getUser, getUserPackages } from "$api";
    import { page } from "$app/stores";
    import type { LoadingState, PackageData, SortMode, User } from "$lib/types";
    import { onMount } from "svelte";
    import { fly } from "svelte/transition";
    import { user as userStore, userPreferencesStore, sortPackages } from "$lib/stores";
    import { admins } from "$lib/data";
    import { getToastStore } from "@skeletonlabs/skeleton";
    import PackageList from "$components/ui/PackageList.svelte";
    import { guessSortMode } from "$lib/utils";
    import TablerIcon from "$components/icons/TablerIcon.svelte";
    import { contextMenu, type ContextMenuItem } from "$lib/contextMenu";
    import TablerIconCheck from "$components/icons/TablerIconCheck.svelte";
    import IconBlank from "$components/icons/IconBlank.svelte";
    import { goto } from "$app/navigation";

    const id = $derived($page.params.id);
    const toasts = getToastStore();
    const showDetails = $derived(($page.url.searchParams.get("showDetails") ?? "false") == "true");

    let loadingState = $state<LoadingState>("loading");
    let user: User | undefined = $state(undefined);
    let packages: PackageData[] = $state([]);

    const downloads = $derived(packages.reduce((a, b) => a + b.downloads, 0));
    const views = $derived(packages.reduce((a, b) => a + b.views, 0));
    const sortedPackages = $derived(sortPackages(packages, $userPreferencesStore.sortBy, false));

    onMount(async () => {
        loadingState = "loading";

        $userPreferencesStore.sortBy = guessSortMode($page.url.searchParams.get("sort") ?? "");

        try {
            user = await getUser(id);
            packages = (await getUserPackages(id)) ?? [];
            loadingState = "ready";
        } catch (_unused) {
            loadingState = "failed";
        }
    });
</script>

<svelte:head>
    <title>{user?.username ?? $_("site.loading")} - KJSPKG</title>
</svelte:head>

{#if loadingState == "loading"}
    <div class="placeholder m-2 mx-auto w-32 animate-pulse"></div>
{:else if loadingState == "ready"}
    <div class="h2 mb-1 flex w-full flex-row items-center justify-between font-bold">
        <div class="flex flex-row items-center justify-start">
            <img
                src="https://avatars.githubusercontent.com/u/{user?.github_id}"
                alt="author's profile"
                class="mr-4 aspect-square h-16 rounded-token"
            />

            <span class="h2 font-bold" in:fly={{ y: 20 }}>{user?.username}</span>
        </div>

        <div class="flex flex-row items-center justify-end">
            {#if user && admins.includes(user.github_id)}
                <span class="variant-filled-error badge">{$_("user.admin")}</span>
            {/if}

            {#if $userStore && user && $userStore.github_id == user.github_id}
                <span class="variant-filled-primary badge ml-2">{$_("user.you")}</span>
            {/if}
        </div>
    </div>

    <div
        class="style-markdown blockquote mb-1 flex w-full select-text flex-row items-center justify-between gap-1 overflow-x-auto p-4 not-italic"
        in:fly={{ y: 20 }}
    >
        <div class="flex h-full w-full flex-col gap-1 overflow-x-auto">
            <span class="text-sm opacity-50">
                <span
                    >{downloads}
                    {downloads == 1
                        ? $_("list.download_singluar")
                        : $_("list.download_plural")}</span
                >
                &bull;
                <span
                    >{views}
                    {views == 1 ? $_("list.view_singular") : $_("list.view_plural")}</span
                >
            </span>
        </div>
    </div>

    <div class="card p-4" in:fly={{ y: 20 }}>
        <dt class="mb-2 text-sm opacity-50">{$_("user.packages")}</dt>

        <div class="flex flex-row items-center justify-between">
            <button
                class="variant-soft-secondary btn mb-4 w-fit hover:variant-filled-primary"
                onclick={() => ($userPreferencesStore.compact = !$userPreferencesStore.compact)}
            >
                <TablerIcon name="layout-dashboard" class="mr-2" />

                <span class="md:inline">
                    {$userPreferencesStore.compact
                        ? $_("search.use_view.list")
                        : $_("search.use_view.compact")}
                </span>
            </button>

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
                                        action: () =>
                                            ($userPreferencesStore.sortBy = name as SortMode),
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
                        : "Unsorted"}
                </button>
            </div>
        </div>

        <div
            class="grid grid-cols-1 gap-2"
            class:lg:grid-cols-2={!$userPreferencesStore.compact}
            class:md:grid-cols-2={$userPreferencesStore.compact}
            class:lg:grid-cols-3={$userPreferencesStore.compact}
        >
            <PackageList
                {showDetails}
                compact={$userPreferencesStore.compact}
                packages={sortedPackages}
            />
        </div>
    </div>
{:else if loadingState == "failed"}
    <!-- <p>Something went wrong (this package doesn't seem to exist)</p> -->
    {(() => {
        toasts.trigger({
            message: "User not found!",
            hideDismiss: true,
            timeout: 5000,
            background: "variant-filled-error",
        });

        history.back();

        return undefined;
    })() || "Please wait, redirecting..."}
{/if}
