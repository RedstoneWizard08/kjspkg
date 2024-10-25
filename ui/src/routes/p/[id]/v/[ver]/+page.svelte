<script lang="ts">
    import { _ } from "svelte-i18n";
    import { page } from "$app/stores";
    import type { LoadingState, PackageData, PackageVersion } from "$lib/types";
    import { fixLoaderName, formatDate } from "$lib/utils";
    import { fly } from "svelte/transition";
    import { onMount } from "svelte";
    import { getPackage, getPackageVersion } from "$api";
    import { getToastStore } from "@skeletonlabs/skeleton";
    import { IconDownload } from "@tabler/icons-svelte";
    import ManagePackage from "$components/ui/ManagePackage.svelte";

    const id = $derived($page.params.id);
    const ver = $derived($page.params.ver);

    let loadingState: LoadingState = $state("loading");
    let pkg: PackageData | undefined = $state(undefined);
    let version: PackageVersion | undefined = $state(undefined);

    onMount(async () => {
        pkg = await getPackage(id);
        version = await getPackageVersion(id, ver);

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
{:else if loadingState == "ready" && pkg && version}
    <ol class="breadcrumb">
        <li class="crumb-separator text-2xl" aria-hidden="true">&lsaquo;</li>
        <li class="crumb">
            <a href="/p/{id}" class="anchor select-text no-underline">{pkg.name}</a>
        </li>
    </ol>

    <h1 class="h2 mb-1 font-bold">
        {version.name ?? "Unnamed Package"}
    </h1>

    <div
        class="style-markdown blockquote mb-1 flex w-full select-text flex-row items-center justify-between gap-1 overflow-x-auto p-4 not-italic"
        in:fly={{ y: 20 }}
    >
        <div class="flex h-full w-full flex-col gap-1 overflow-x-auto">
            <span class="select-text *:select-text">
                <span class="font-bold">Version Number:</span>
                {version.version_number}
            </span>

            <span class="text-sm opacity-50">
                <span
                    >{version.downloads}
                    {version.downloads == 1
                        ? $_("list.download_singluar")
                        : $_("list.download_plural")}</span
                >
            </span>
        </div>
        <a
            href="/api/v1/packages/{id}/versions/{ver}/download"
            class="flex flex-row items-center justify-center rounded-full p-2 transition-all hover:variant-filled-primary"
        >
            <IconDownload />
        </a>
    </div>

    <div class="grid grid-cols-1 gap-2 lg:grid-cols-2">
        <div class="card p-4" in:fly={{ y: 20 }}>
            <dt class="text-sm opacity-50">{$_("package.loaders_title")}</dt>
            <dd class="mt-2 flex flex-wrap gap-1">
                {#each version.loaders as item}
                    <span class="variant-filled-primary badge select-text"
                        >{fixLoaderName(item)}</span
                    >
                {/each}
            </dd>
        </div>

        <div class="card p-4" in:fly={{ y: 20 }}>
            <dt class="text-sm opacity-50">{$_("package.minecraft_title")}</dt>
            <dd class="mt-2 flex flex-wrap gap-1">
                {#each version.minecraft as item}
                    <span class="variant-filled-primary badge select-text">{item}</span>
                {/each}
            </dd>
        </div>

        <!-- <CodeBlock
			language="Install package"
			code={'kjspkg install ' + id}
			background="variant-soft w-full"
			buttonCopied="ok have fun"
		/> -->

        <div class="card space-y-2 p-4 md:block" in:fly={{ y: 20 }}>
            <dt class="text-sm opacity-50">{$_("package.manage_package")}</dt>
            <dd class="flex flex-col gap-1">
                <ManagePackage name={pkg.slug ?? "no-name"} {version} link={pkg.issues} />
            </dd>
        </div>

        <div class="flex flex-col space-y-2 md:block" in:fly={{ y: 20 }}>
            <div class="card space-y-2 p-4 md:block" in:fly={{ y: 20 }}>
                <dt class="text-sm opacity-50">{$_("package.version.published")}</dt>
                <dd class="flex flex-col gap-1">{formatDate(new Date(version.created_at))}</dd>
            </div>
            <div class="card space-y-2 p-4 md:block" in:fly={{ y: 20 }}>
                <dt class="text-sm opacity-50">{$_("package.version.updated")}</dt>
                <dd class="flex flex-col gap-1">{formatDate(new Date(version.updated_at))}</dd>
            </div>
        </div>

        {#if version.changelog}
            <section class="card h-fit space-y-4 p-4 lg:col-span-2" in:fly={{ y: 20 }}>
                <dt class="text-sm opacity-50">
                    {$_("package.version.changelog")}
                </dt>
                <dd class="style-markdown flex select-text flex-col items-start *:select-text">
                    {@html version.changelog}
                </dd>
            </section>
        {/if}
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
