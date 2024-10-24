<script lang="ts">
    import { _ } from "svelte-i18n";
    import { page } from "$app/stores";
    import type { LoadingState, PackageData, PackageVersion } from "$lib/types";
    import { fixLoaderName, getKubeJS, getLoaders, getMinecraft, markdownInline } from "$lib/utils";
    import { fly } from "svelte/transition";
    import { onMount } from "svelte";
    import { getPackage, getPackageVersions } from "$api";
    import { getToastStore } from "@skeletonlabs/skeleton";
    import ManagePackage from "$components/ui/ManagePackage.svelte";
    import { base } from "$app/paths";
    import Version from "$components/ui/Version.svelte";

    const id = $derived($page.params.id);
    let loadingState: LoadingState = $state("loading");
    let pkg: PackageData | undefined = $state(undefined);
    let versions: PackageVersion[] = $state([]);
    const loaders = $derived(getLoaders(versions));
    const kubejs = $derived(getKubeJS(versions));
    const minecraft = $derived(getMinecraft(versions));

    const sortedVersions = $derived(
        versions.sort(
            (a, b) => new Date(a.created_at).getTime() - new Date(b.created_at).getTime(),
        ),
    );

    const latest = $derived(sortedVersions[0]);

    onMount(async () => {
        pkg = await getPackage(id);
        versions = (await getPackageVersions(id)) ?? [];

        if (pkg) {
            loadingState = "ready";
        } else {
            loadingState = "failed";
        }
    });
</script>

<svelte:head>
    <title>{pkg?.name ?? "no-name"} - KJSPKG Lookup</title>
</svelte:head>

{#if loadingState == "loading"}
    <div class="placeholder m-2 mx-auto w-32 animate-pulse"></div>
{:else if loadingState == "ready" && pkg}
    <h1 class="h2 font-bold">
        {pkg.name ?? "This package"}
    </h1>

    <div
        class="style-markdown blockquote flex w-full select-text flex-col gap-1 overflow-x-auto p-4"
        in:fly={{ y: 20 }}
    >
        <span class="select-text *:select-text">
            {@html markdownInline(pkg.description)}
        </span>

        <span class="text-sm opacity-50">
            <span
                >{pkg.downloads}
                {pkg.downloads == 1
                    ? $_("list.download_singluar")
                    : $_("list.download_plural")}</span
            >
            &bull;
            <span
                >{pkg.views}
                {pkg.views == 1 ? $_("list.view_singular") : $_("list.view_plural")}</span
            >
        </span>
    </div>

    {#if pkg.source || pkg.issues || pkg.wiki}
        <span class="card p-4">
            {#if pkg.source}
                <a href={pkg.source} class="anchor select-text no-underline" target="_blank">
                    Source Code
                </a>
            {/if}
            {#if (pkg.source && pkg.issues) || (pkg.source && pkg.wiki)}
                &bull;
            {/if}
            {#if pkg.issues}
                <a href={pkg.issues} class="anchor select-text no-underline" target="_blank">
                    Issue Tracker
                </a>
            {/if}
            {#if (pkg.issues && pkg.wiki) || (pkg.source && pkg.wiki)}
                &bull;
            {/if}
            {#if pkg.wiki}
                <a href={pkg.wiki} class="anchor select-text no-underline" target="_blank">
                    Wiki
                </a>
            {/if}
        </span>
    {/if}

    <div class="grid grid-cols-1 gap-2 lg:grid-cols-2">
        <!-- TODO: href={base + `/s?q=@author:${locatorInfo[1]}`} -->
        <div class="card p-4" in:fly={{ y: 20 }}>
            <dt class="mb-2 text-sm opacity-50">{$_("package.created_by")}</dt>
            {#each pkg.authors as author}
                <a
                    class="card hover:variant-soft-primary flex p-4"
                    href="{base}/s?q={author.username}"
                    in:fly={{ y: 20 }}
                >
                    <img
                        src="https://avatars.githubusercontent.com/u/{author.github_id}"
                        alt="author's profile afirst child cssvatar"
                        class="rounded-token my-auto mr-4 aspect-square h-8"
                    />
                    <!-- <dd class="select-text font-bold"> -->
                    {author.username}
                    <!-- {locatorInfo[1] != thisPackage.author ? `(${locatorInfo[1]})` : ""} -->
                    <!-- </dd> -->
                </a>
            {/each}
        </div>

        <div class="card p-4" in:fly={{ y: 20 }}>
            <dt class="text-sm opacity-50">{$_("package.available_for")}</dt>
            <dd class="mt-2 flex flex-wrap gap-1">
                {#each loaders as loader}
                    <span class="variant-filled-primary badge select-text"
                        >{fixLoaderName(loader)}</span
                    >
                {/each}
            </dd>
            <dd class="mt-2 flex flex-wrap gap-1">
                {#each minecraft as version}
                    <span class="variant-filled-primary badge select-text">{version}</span>
                {/each}
            </dd>
        </div>

        <!-- <CodeBlock
			language="Install package"
			code={'kjspkg install ' + id}
			background="variant-soft w-full"
			buttonCopied="ok have fun"
		/> -->

        <!-- <div class="card hidden space-y-2 p-4 md:block" in:fly={{ y: 20 }}>
            <dt class="text-sm opacity-50">{$_("package.manage_package")}</dt>
            <dd class="flex flex-col gap-1">
                <ManagePackage name={pkg.slug ?? "no-name"} version={latest} link={pkg.issues} />
            </dd>
        </div> -->

        {#if sortedVersions.length > 0}
            <div class="card h-fit space-y-2 p-4 lg:col-span-2" in:fly|global={{ y: 20 }}>
                <dt class="text-sm opacity-50">{$_("package.versions")}</dt>

                <dd class="flex w-full gap-1">
                    <dl class="list-dl w-full">
                        {#each sortedVersions as version}
                            <Version {version} pkg={pkg.slug} />
                        {/each}
                    </dl>
                </dd>
            </div>
        {/if}

        <section class="card h-fit space-y-4 p-4 lg:col-span-2" in:fly={{ y: 20 }}>
            <dt class="text-sm opacity-50">
                {$_("package.readme_file")}
            </dt>
            <dd class="style-markdown flex select-text flex-col items-start *:select-text">
                {@html pkg.readme}
            </dd>
        </section>
    </div>
{:else if loadingState == "failed"}
    <!-- <p>Something went wrong (this package doesn't seem to exist)</p> -->
    {(() => {
        getToastStore().trigger({
            message: `Package Broken`,
            hideDismiss: true,
            timeout: 5000,
            background: "variant-filled-error",
        });

        history.back();

        return undefined;
    })() || "Please wait, redirecting..."}
{/if}
