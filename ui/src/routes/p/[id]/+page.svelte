<script lang="ts">
    import { beforeNavigate, goto } from "$app/navigation";
    import { _ } from "svelte-i18n";
    import { page } from "$app/stores";
    import type { LoadingState, PackageData, PackageVersion } from "$lib/types";
    import { fixLoaderName, getLoaders, getMinecraft, markdown, markdownInline } from "$lib/utils";
    import { fly } from "svelte/transition";
    import { onMount } from "svelte";
    import { deletePackage, getPackage, getPackageVersions, updatePackage } from "$api";
    import { getToastStore, getModalStore, ProgressRadial } from "@skeletonlabs/skeleton";
    import { base } from "$app/paths";
    import Version from "$components/ui/Version.svelte";
    import { currentPackage, forceUpdatePackagesStore, user } from "$lib/stores";
    import { Carta, MarkdownEditor } from "carta-md";
    import TablerIcon from "$components/icons/TablerIcon.svelte";

    const maxVersions = 10;

    const id = $derived($page.params.id);
    const toasts = getToastStore();
    const modals = getModalStore();

    let loadingState: LoadingState = $state("loading");
    let versions: PackageVersion[] = $state([]);

    let readme = $state("");
    let name = $state("");
    let repo = $state("");
    let issues = $state("");
    let wiki = $state("");

    let editing = $state(false);
    let saving = $state(false);

    const loaders = $derived(getLoaders(versions));
    const minecraft = $derived(getMinecraft(versions));

    const hasRepo = $derived(repo != "");
    const hasIssues = $derived(issues != "");
    const hasWiki = $derived(wiki != "");

    const realRepo = $derived(repo != "" ? repo : undefined);
    const realIssues = $derived(issues != "" ? issues : undefined);
    const realWiki = $derived(wiki != "" ? wiki : undefined);

    const canEdit = $derived(
        $currentPackage &&
            $user &&
            !!($currentPackage as PackageData).authors.find((v) => v.id == $user.id),
    );

    const sortedVersions = $derived(
        versions.sort(
            (a, b) => new Date(a.created_at).getTime() - new Date(b.created_at).getTime(),
        ),
    );

    const editor = new Carta();

    const toggleEditing = async () => {
        if (editing) {
            saving = true;

            await updatePackage(id, {
                name,
                readme,
                source: realRepo,
                issues: realIssues,
                wiki: realWiki,
            });

            saving = false;
        }

        editing = !editing;
    };

    const openAddingModal = () => {
        modals.trigger({
            type: "component",
            component: "addAuthor",
        });
    };

    const openUploadModal = () => {
        modals.trigger({
            type: "component",
            component: "uploadVersion",
        });
    };

    const onDeleteVersion = async () => {
        versions = (await getPackageVersions(id)) ?? [];
    };

    const onDeletePackage = async () => {
        try {
            await deletePackage(id);
        } catch (e) {
            toasts.trigger({
                message: `Failed to delete package: ${e}`,
                background: "variant-filled-error",
            });
        }

        await forceUpdatePackagesStore();
        goto("/");
    };

    onMount(async () => {
        $currentPackage = await getPackage(id);
        versions = (await getPackageVersions(id)) ?? [];

        if ($currentPackage) {
            name = $currentPackage.name;
            readme = $currentPackage.readme;
            repo = $currentPackage.source ?? "";
            issues = $currentPackage.issues ?? "";
            wiki = $currentPackage.wiki ?? "";
            loadingState = "ready";
        } else {
            loadingState = "failed";
        }
    });

    beforeNavigate(() => {
        $currentPackage = undefined;
    });
</script>

<svelte:head>
    <title>{$currentPackage?.name ?? "Loading"} - KJSPKG</title>
</svelte:head>

{#if loadingState == "loading"}
    <div class="placeholder m-2 mx-auto w-32 animate-pulse"></div>
{:else if loadingState == "ready" && $currentPackage}
    {#if saving}
        <div
            class="fixed left-0 right-0 top-0 z-50 flex h-full w-full flex-row items-center justify-center bg-primary-900 bg-opacity-25 text-white"
            in:fly={{ y: 20 }}
            out:fly={{ y: 20 }}
        >
            <ProgressRadial width="w-20" />
        </div>
    {/if}

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
                {@html markdownInline($currentPackage.description)}
            </span>

            <span class="text-sm opacity-50">
                <span
                    >{$currentPackage.downloads}
                    {$currentPackage.downloads == 1
                        ? $_("list.download_singluar")
                        : $_("list.download_plural")}</span
                >
                &bull;
                <span
                    >{$currentPackage.views}
                    {$currentPackage.views == 1
                        ? $_("list.view_singular")
                        : $_("list.view_plural")}</span
                >
            </span>
        </div>

        {#if canEdit}
            <button
                onclick={onDeletePackage}
                class="variant-glass-error flex flex-row items-center justify-center rounded-full p-2 transition-all hover:variant-filled-error"
            >
                <TablerIcon name="trash" />
            </button>
            <button
                onclick={toggleEditing}
                class="flex flex-row items-center justify-center rounded-full p-2 transition-all hover:variant-filled-primary"
            >
                {#if editing}
                    <TablerIcon name="device-floppy" />
                {:else}
                    <TablerIcon name="pencil" />
                {/if}
            </button>
        {/if}
    </div>

    {#if editing}
        <span class="input-group p-0">
            <input
                type="text"
                class="indent-2"
                placeholder="Code Repository URL"
                bind:value={repo}
            />
        </span>
        <span class="input-group p-0">
            <input
                type="text"
                class="indent-2"
                placeholder="Issue Tracker URL"
                bind:value={issues}
            />
        </span>
        <span class="input-group p-0">
            <input type="text" class="indent-2" placeholder="Wiki URL" bind:value={wiki} />
        </span>
    {:else if hasRepo || hasIssues || hasWiki}
        <span class="card p-4">
            {#if hasRepo}
                <a href={repo} class="anchor select-text no-underline" target="_blank">
                    {$_("package.source")}
                </a>
            {/if}
            {#if (hasRepo && hasIssues) || (hasRepo && hasWiki)}
                &bull;
            {/if}
            {#if hasIssues}
                <a href={issues} class="anchor select-text no-underline" target="_blank">
                    {$_("package.issues")}
                </a>
            {/if}
            {#if (hasIssues && hasWiki) || (hasRepo && hasWiki)}
                &bull;
            {/if}
            {#if hasWiki}
                <a href={wiki} class="anchor select-text no-underline" target="_blank">
                    {$_("package.wiki")}
                </a>
            {/if}
        </span>
    {/if}

    <div class="grid grid-cols-1 gap-2 lg:grid-cols-2">
        <div class="card p-4" in:fly={{ y: 20 }}>
            <dt class="mb-2 text-sm opacity-50">{$_("package.created_by")}</dt>

            {#each $currentPackage.authors as author}
                <a
                    class="card mb-2 flex flex-row items-center p-2 hover:variant-soft-primary"
                    href="{base}/u/{author.username}"
                    in:fly={{ y: 20 }}
                >
                    <img
                        src="https://avatars.githubusercontent.com/u/{author.github_id}"
                        alt="author's profile afirst child cssvatar"
                        class="my-auto mr-4 aspect-square h-8 rounded-token"
                    />
                    {author.username}
                </a>
            {/each}

            {#if editing}
                <button
                    class="card flex w-full p-2 transition-all hover:variant-soft-primary"
                    in:fly={{ y: 20 }}
                    onclick={openAddingModal}
                >
                    <TablerIcon name="plus" />
                    {$_("modal.add_author.trigger")}
                </button>
            {/if}
        </div>

        <div class="card p-4" in:fly={{ y: 20 }}>
            <dt class="text-sm opacity-50">{$_("package.available_for")}</dt>

            {#if loaders.length > 0 || minecraft.length > 0}
                {#if loaders.length > 0}
                    <dd class="mt-2 flex flex-wrap gap-1">
                        {#each loaders as loader}
                            <span class="variant-filled-primary badge select-text"
                                >{fixLoaderName(loader)}</span
                            >
                        {/each}
                    </dd>
                {/if}
                {#if minecraft.length > 0}
                    <dd class="mt-2 flex flex-wrap gap-1">
                        {#if minecraft.length > maxVersions}
                            {#each minecraft.slice(0, maxVersions) as version}
                                <span class="variant-filled-primary badge select-text"
                                    >{version}</span
                                >
                            {/each}
                            <span class="variant-filled-primary badge select-text">...</span>
                        {:else}
                            {#each minecraft as version}
                                <span class="variant-filled-primary badge select-text"
                                    >{version}</span
                                >
                            {/each}
                        {/if}
                    </dd>
                {/if}
            {:else}
                <dd class="mt-2 flex flex-wrap gap-1">
                    <span class="variant-filled-primary badge select-text"
                        >{$_("package.unknown")}</span
                    >
                </dd>
            {/if}
        </div>

        {#if sortedVersions.length > 0 || canEdit}
            <div class="card h-fit space-y-2 p-4 lg:col-span-2" in:fly|global={{ y: 20 }}>
                <dt class="text-sm opacity-50">{$_("package.versions")}</dt>

                <dd class="flex w-full gap-1">
                    <dl class="list-dl w-full">
                        {#each sortedVersions as version}
                            <Version
                                {version}
                                pkg={$currentPackage.slug}
                                {editing}
                                onDelete={onDeleteVersion}
                            />
                        {/each}
                    </dl>
                </dd>

                {#if canEdit}
                    <button
                        class="variant-soft-secondary btn w-full transition-all hover:variant-soft-primary"
                        in:fly={{ y: 20 }}
                        onclick={openUploadModal}
                    >
                        <TablerIcon name="upload" class="mr-2" />
                        {$_("package.upload_version")}
                    </button>
                {/if}
            </div>
        {/if}

        <section class="card h-fit p-4 lg:col-span-2" in:fly={{ y: 20 }}>
            <dt class="text-sm opacity-50">
                {$_("package.readme_file")}
            </dt>
            {#if editing}
                <dd class="flex w-full flex-col items-start" in:fly={{ y: 20 }}>
                    <MarkdownEditor carta={editor} bind:value={readme} mode="tabs" />
                </dd>
            {:else}
                <dd
                    class="style-markdown flex select-text flex-col items-start *:select-text"
                    in:fly={{ y: 20 }}
                >
                    {@html markdown(readme)}
                </dd>
            {/if}
        </section>
    </div>
{:else if loadingState == "failed"}
    <!-- <p>Something went wrong (this package doesn't seem to exist)</p> -->
    {(() => {
        toasts.trigger({
            message: `Package Broken`,
            hideDismiss: true,
            timeout: 5000,
            background: "variant-filled-error",
        });

        history.back();

        return undefined;
    })() || "Please wait, redirecting..."}
{/if}
