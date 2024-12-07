<script lang="ts">
    import { _ } from "svelte-i18n";
    import { page } from "$app/stores";
    import type { PackageData, PackageVersion } from "$lib/types";
    import { fixLoaderName, markdown } from "$lib/utils";
    import { onMount } from "svelte";
    import { getPackageVersion } from "$api";
    import { currentPackage, user } from "$lib/stores";
    import { siteConfig } from "$lib/config";
    import Icon from "@iconify/svelte";
    import { downloadFile } from "$lib/download";
    import { copyText } from "$lib/clipboard";
    import { getToastStore } from "@skeletonlabs/skeleton";
    import { tryAggregateVersions } from "$lib/vers";

    const maxVersions = 10;
    const id = $derived($page.params.id);
    const ver = $derived($page.params.ver);
    const toasts = getToastStore();

    let done = $state(false);
    let downloading = $state(false);
    let version: PackageVersion | undefined = $state(undefined);
    let name = $state("");
    let changelog = $state<string | undefined>(undefined);

    const loaders = $derived((version as PackageVersion | undefined)?.loaders ?? []);
    const gameVersions = $derived((version as PackageVersion | undefined)?.game_versions ?? []);
    const aggVersions = $derived(tryAggregateVersions(gameVersions));

    const canEdit = $derived(
        ($currentPackage &&
            $user &&
            !!($currentPackage as PackageData).authors.find((v) => v.id == $user.id)) ||
            ($user && $user.admin),
    );

    onMount(async () => {
        version = await getPackageVersion(id, ver);

        if (!$currentPackage || !version) return;

        name = version.name;
        changelog = version.changelog;
    });

    let doneTimeout: number | undefined;

    const directDownload = async (ev: Event) => {
        ev.preventDefault();
        ev.stopPropagation();

        if (!version || !$currentPackage) return;

        downloading = true;

        const fileName = `${$currentPackage.slug}_${version.version_number}.mhpkg`;

        await downloadFile(
            `/api/v1/packages/${$currentPackage}/versions/${version.id}/download`,
            fileName,
        );

        downloading = false;
        done = true;

        if (doneTimeout) clearTimeout(doneTimeout);

        doneTimeout = setTimeout(() => {
            done = false;
        }, 1000) as any;
    };

    const copyVersionId = async () => {
        if (!version) return;

        await copyText(version.id.toString(), toasts);
    };
</script>

<svelte:head>
    <title>{version?.name ?? $_("site.loading")} - {siteConfig.siteName}</title>
</svelte:head>

<div class="card flex w-full flex-col items-start justify-start p-4">
    <div class="flex w-full flex-row items-center justify-between">
        <span class="text-xl font-bold">{name}</span>

        <div class="flex flex-row items-center justify-end gap-2">
            {#if canEdit}
                <a
                    aria-label="Edit"
                    href="/p/{id}/edit/versions/edit/{ver}"
                    class="flex flex-row items-center justify-center rounded-full p-2 transition-all hover:variant-filled-primary"
                >
                    <Icon icon="tabler:pencil" height="24" />
                </a>
            {/if}

            <button
                type="button"
                class="btn p-2 transition-all hover:variant-filled-primary"
                onclick={directDownload}
            >
                {#if done}
                    <Icon icon="tabler:check" height="24" />
                {:else if downloading}
                    <Icon icon="tabler:loader-2" height="24" class="animate-spin" />
                {:else}
                    <Icon icon="tabler:download" height="24" />
                {/if}
            </button>
        </div>
    </div>

    <div class="mt-2 flex w-full flex-row items-center justify-between">
        <span class="text-sm opacity-80">
            <span class="font-bold">{$_("package.version.prefix")}</span>
            {version?.version_number}
        </span>

        <span class="text-sm opacity-50">
            <span
                >{version?.downloads}
                {version?.downloads == 1
                    ? $_("list.download_singluar")
                    : $_("list.download_plural")}</span
            >
            &bull;
            {$_("id.version")}&nbsp;
            <button class="anchor select-text no-underline" onclick={copyVersionId}
                >{version?.id}</button
            >
        </span>
    </div>
</div>

<div class="card grid w-full grid-cols-[1fr_1fr] gap-2 p-4">
    <div class="flex w-full flex-col items-start justify-start gap-2">
        <dt class="text-sm opacity-50">{$_("package.loaders_title")}</dt>

        {#if loaders.length > 0}
            <dd class="flex flex-wrap gap-1">
                {#each loaders as loader}
                    <span class="variant-filled-primary badge select-text"
                        >{fixLoaderName(loader)}</span
                    >
                {/each}
            </dd>
        {:else}
            <dd class="flex flex-wrap gap-1">
                <span class="variant-filled-primary badge select-text">{$_("package.unknown")}</span
                >
            </dd>
        {/if}
    </div>

    <div class="flex w-full flex-col items-start justify-start gap-2">
        <dt class="text-sm opacity-50">{$_("package.available_for")}</dt>

        {#if gameVersions.length > 0}
            <dd class="flex flex-wrap gap-1">
                {#if aggVersions.length > maxVersions}
                    {#each aggVersions.slice(0, maxVersions) as version}
                        <span class="variant-filled-primary badge select-text">{version}</span>
                    {/each}
                    <span class="variant-filled-primary badge select-text">...</span>
                {:else}
                    {#each aggVersions as version}
                        <span class="variant-filled-primary badge select-text">{version}</span>
                    {/each}
                {/if}
            </dd>
        {:else}
            <dd class="flex flex-wrap gap-1">
                <span class="variant-filled-primary badge select-text">{$_("package.unknown")}</span
                >
            </dd>
        {/if}
    </div>
</div>

<section class="card h-fit w-full p-4">
    <dt class="mb-2 text-sm opacity-50">
        {$_("package.version.changelog")}
    </dt>

    <dd class="style-markdown flex select-text flex-col items-start *:select-text">
        {@html markdown(changelog ?? "")}
    </dd>
</section>
