<script lang="ts">
    import { _ } from "svelte-i18n";
    import { getUser, getUserPackages } from "$api";
    import { page } from "$app/stores";
    import type { LoadingState, PackageData, User } from "$lib/types";
    import { onMount } from "svelte";
    import { fly } from "svelte/transition";
    import { base } from "$app/paths";
    import { user as userStore } from "$lib/stores";
    import { admins } from "$lib/data";
    import { getToastStore } from "@skeletonlabs/skeleton";

    const id = $derived($page.params.id);
    const toasts = getToastStore();

    let loadingState = $state<LoadingState>("loading");
    let user: User | undefined = $state(undefined);
    let packages: PackageData[] = $state([]);

    const downloads = $derived(packages.reduce((a, b) => a + b.downloads, 0));
    const views = $derived(packages.reduce((a, b) => a + b.views, 0));

    onMount(async () => {
        loadingState = "loading";

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
    <title>{user?.username ?? "Loading"} - KJSPKG Lookup</title>
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

        {#each packages as pkg}
            <a
                class="card mb-2 flex flex-row items-center justify-between p-4 hover:variant-soft-primary"
                href="{base}/p/{pkg.slug}"
                in:fly={{ y: 20 }}
            >
                <span>{pkg.description}</span>

                <span class="text-sm opacity-50"
                    ><span
                        >{pkg.downloads}
                        {pkg.downloads == 1
                            ? $_("list.download_singluar")
                            : $_("list.download_plural")}</span
                    >
                    &bull;
                    <span
                        >{pkg.views}
                        {pkg.views == 1 ? $_("list.view_singular") : $_("list.view_plural")}</span
                    ></span
                >
            </a>
        {/each}
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
