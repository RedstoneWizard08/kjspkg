<script lang="ts">
    import { _ } from "svelte-i18n";
    import { page } from "$app/stores";
    import type { LoadingState, PackageData, PackageVersion } from "$lib/types";
    import { fixLoaderName, formatDate, markdown } from "$lib/utils";
    import { fly } from "svelte/transition";
    import { onMount } from "svelte";
    import { getPackage, getPackageVersion, updateVersion } from "$api";
    import { getToastStore, ProgressRadial } from "@skeletonlabs/skeleton";
    import TablerIcon from "$components/icons/TablerIcon.svelte";
    import { currentPackage, user } from "$lib/stores";
    import { beforeNavigate } from "$app/navigation";
    import { Carta, MarkdownEditor } from "carta-md";
    import { tryAggregateVersions } from "$lib/vers";
    import { siteConfig } from "$lib/config";
    import { copyText } from "$lib/clipboard";

    const maxVersions = 10;

    const id = $derived($page.params.id);
    const ver = $derived($page.params.ver);
    const editor = new Carta();
    const toasts = getToastStore();

    let editing = $state(false);
    let saving = $state(false);

    let repo = $state("");
    let issues = $state("");
    let wiki = $state("");

    let loadingState: LoadingState = $state("loading");
    let version: PackageVersion | undefined = $state(undefined);

    let name = $state("");
    let changelog = $state<string | undefined>(undefined);

    const hasRepo = $derived(repo != "");
    const hasIssues = $derived(issues != "");
    const hasWiki = $derived(wiki != "");

    const canEdit = $derived(
        $currentPackage &&
            $user &&
            !!($currentPackage as PackageData).authors.find((v) => v.id == $user.id),
    );

    const toggleEditing = async () => {
        if (editing) {
            saving = true;

            await updateVersion(id, ver, { name });

            saving = false;
        }

        editing = !editing;
    };

    const copyPackageId = async () => {
        if (!$currentPackage) return;

        await copyText($currentPackage.id.toString(), toasts);
    };

    const copyVersionId = async () => {
        if (!version) return;

        await copyText(version.id.toString(), toasts);
    };

    onMount(async () => {
        $currentPackage = await getPackage(id);
        version = await getPackageVersion(id, ver);

        if ($currentPackage && version) {
            name = version.name;
            changelog = version.changelog;
            loadingState = "ready";
            repo = $currentPackage.source ?? "";
            issues = $currentPackage.issues ?? "";
            wiki = $currentPackage.wiki ?? "";
        } else {
            loadingState = "failed";
        }
    });

    beforeNavigate(() => {
        $currentPackage = undefined;
    });

    const aggVersions = $derived(
        tryAggregateVersions((version as PackageVersion | undefined)?.game_versions ?? []),
    );
</script>

<svelte:head>
    <title>{version?.name ?? $_("site.loading")} - {siteConfig.siteName}</title>
</svelte:head>

{#if loadingState == "loading"}
    <div class="placeholder m-2 mx-auto w-32 animate-pulse"></div>
{:else if loadingState == "ready" && $currentPackage && version}
    {#if saving}
        <div
            class="fixed left-0 right-0 top-0 z-50 flex h-full w-full flex-row items-center justify-center bg-primary-900 bg-opacity-25 text-white"
            in:fly={{ y: 20 }}
            out:fly={{ y: 20 }}
        >
            <ProgressRadial width="w-20" />
        </div>
    {/if}

    <ol class="breadcrumb">
        <li class="crumb-separator text-2xl" aria-hidden="true">&lsaquo;</li>
        <li class="crumb">
            <a href="/p/{id}" class="anchor select-text no-underline">{$currentPackage.name}</a>
        </li>
    </ol>

    <h1 class="h2 mb-1 font-bold">
        {#if editing}
            <input
                in:fly={{ y: 20 }}
                type="text"
                bind:value={name}
                class="input variant-form-material w-full border-primary-900"
            />
        {:else}
            <span class="h2 font-bold" in:fly={{ y: 20 }}>{name}</span>
        {/if}
    </h1>

    <div
        class="style-markdown blockquote mb-1 flex w-full select-text flex-row items-center justify-between gap-1 overflow-x-auto p-4 not-italic"
        in:fly={{ y: 20 }}
    >
        <div class="flex h-full w-full flex-col gap-1 overflow-x-auto">
            <span class="select-text *:select-text">
                <span class="font-bold">{$_("package.version.prefix")}</span>
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

        <div class="flex flex-row items-center justify-end">
            {#if canEdit}
                <button
                    onclick={toggleEditing}
                    class="mr-2 flex flex-row items-center justify-center rounded-full p-2 transition-all hover:variant-filled-primary"
                >
                    {#if editing}
                        <TablerIcon name="device-floppy" />
                    {:else}
                        <TablerIcon name="pencil" />
                    {/if}
                </button>
            {/if}

            <a
                href="/api/v1/packages/{id}/versions/{ver}/download"
                download={version.file_name}
                class="flex flex-row items-center justify-center rounded-full p-2 transition-all hover:variant-filled-primary"
            >
                <TablerIcon name="download" />
            </a>
        </div>
    </div>

    <span class="card flex flex-row items-center justify-between p-4">
        <div class="flex flex-row items-center justify-start">
            {#if hasRepo}
                <a href={repo} class="anchor select-text no-underline" target="_blank">
                    {$_("package.source")}
                </a>
            {/if}
            {#if (hasRepo && hasIssues) || (hasRepo && hasWiki)}
                &nbsp;&bull;&nbsp;
            {/if}
            {#if hasIssues}
                <a href={issues} class="anchor select-text no-underline" target="_blank">
                    {$_("package.issues")}
                </a>
            {/if}
            {#if (hasIssues && hasWiki) || (hasRepo && hasWiki)}
                &nbsp;&bull;&nbsp;
            {/if}
            {#if hasWiki}
                <a href={wiki} class="anchor select-text no-underline" target="_blank">
                    {$_("package.wiki")}
                </a>
            {/if}
        </div>

        <div class="flex flex-row items-center justify-end">
            <span class="flex flex-row items-center justify-end">
                {$_(`id.${siteConfig.type}`)}&nbsp;
                <button class="anchor select-text no-underline" onclick={copyPackageId}
                    >{$currentPackage.id}</button
                >
            </span>

            &nbsp;&bull;&nbsp;

            <span class="flex flex-row items-center justify-end">
                {$_("id.version")}&nbsp;
                <button class="anchor select-text no-underline" onclick={copyVersionId}
                    >{version.id}</button
                >
            </span>
        </div>
    </span>

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
            <dt class="text-sm opacity-50">{$_("package.game_title")}</dt>
            <dd class="mt-2 flex flex-wrap gap-1">
                {#if aggVersions.length > maxVersions}
                    {#each aggVersions.slice(0, maxVersions) as item}
                        <span class="variant-filled-primary badge select-text">{item}</span>
                    {/each}
                    <span class="variant-filled-primary badge select-text">...</span>
                {:else}
                    {#each aggVersions as item}
                        <span class="variant-filled-primary badge select-text">{item}</span>
                    {/each}
                {/if}
            </dd>
        </div>

        <div class="card space-y-2 p-4 md:block" in:fly={{ y: 20 }}>
            <dt class="text-sm opacity-50">{$_("package.version.published")}</dt>
            <dd class="flex flex-col gap-1">{formatDate(new Date(version.created_at))}</dd>
        </div>

        <div class="card space-y-2 p-4 md:block" in:fly={{ y: 20 }}>
            <dt class="text-sm opacity-50">{$_("package.version.updated")}</dt>
            <dd class="flex flex-col gap-1">{formatDate(new Date(version.updated_at))}</dd>
        </div>

        {#if changelog || editing}
            <section class="card h-fit p-6 lg:col-span-2" in:fly={{ y: 20 }}>
                <dt class="text-sm opacity-50">
                    {$_("package.version.changelog")}
                </dt>
                {#if editing}
                    <dd class="flex w-full flex-col items-start" in:fly={{ y: 20 }}>
                        <MarkdownEditor carta={editor} bind:value={changelog} mode="tabs" />
                    </dd>
                {:else if changelog}
                    <dd
                        class="style-markdown flex select-text flex-col items-start *:select-text"
                        in:fly={{ y: 20 }}
                    >
                        {@html markdown(changelog)}
                    </dd>
                {/if}
            </section>
        {/if}
    </div>
{:else if loadingState == "failed"}
    <!-- <p>Something went wrong (this package doesn't seem to exist)</p> -->
    {(() => {
        getToastStore().trigger({
            message: `Mod/Package Broken`,
            hideDismiss: true,
            timeout: 5000,
            background: "variant-filled-error",
        });

        history.back();

        return undefined;
    })() || "Please wait, redirecting..."}
{/if}
