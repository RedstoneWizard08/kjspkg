<script lang="ts">
    import { _ } from "svelte-i18n";
    import { onMount } from "svelte";
    import { createPackage, getPackage } from "$api";
    import { currentPackage, editSaving, forceUpdatePackagesStore } from "$lib/stores";
    import Icon from "@iconify/svelte";
    import {
        Autocomplete,
        getToastStore,
        popup,
        type AutocompleteOption,
        type PopupSettings,
    } from "@skeletonlabs/skeleton";
    import { getLicenses } from "$lib/licenses";
    import type { ProjectVisibility } from "$lib/types";
    import { goto } from "$app/navigation";
    import { createSlug } from "$lib/utils";
    import { Carta, MarkdownEditor } from "carta-md";
    import { siteConfig } from "$lib/config";

    let name = $state("");
    let slug = $state("");
    let readme = $state("");
    let description = $state("");
    let repo = $state("");
    let issues = $state("");
    let wiki = $state("");
    let license = $state("");
    let visibility = $state<ProjectVisibility>("Public");
    let allLicenses = $state<AutocompleteOption<string, string>[]>([]);
    let slugError = $state(false);

    const editor = new Carta();
    const toasts = getToastStore();
    const realRepo = $derived(repo != "" ? repo : undefined);
    const realIssues = $derived(issues != "" ? issues : undefined);
    const realWiki = $derived(wiki != "" ? wiki : undefined);
    const realLicense = $derived(license != "" ? license : undefined);

    onMount(async () => {
        if (!$currentPackage) return;

        allLicenses = (await getLicenses()).map((v) => ({ value: v, label: v }));
    });

    const save = async () => {
        $editSaving = true;

        const data = await createPackage({
            name,
            slug,
            visibility,
            source: realRepo,
            issues: realIssues,
            wiki: realWiki,
            license: realLicense,
            readme,
            description,
        });

        if (!data) {
            $editSaving = false;

            toasts.trigger({
                message: `Error creating your project!`,
                hideDismiss: true,
                timeout: 5000,
                background: "variant-filled-error",
            });

            return;
        }

        await forceUpdatePackagesStore();

        $editSaving = false;
        goto(`/p/${data?.id}`);
    };

    const licensesPopup: PopupSettings = {
        event: "focus-click",
        target: "licensesAutocomplete",
        placement: "bottom",
    };

    const onLicenseSelect = (ev: CustomEvent<AutocompleteOption<string, string>>) => {
        license = ev.detail.value;
    };

    const updateSlug = async () => {
        slugError = false;
        slug = createSlug(name);
        slugError = !!(await getPackage(slug));
    };
</script>

<svelte:head>
    <title>Create Project - {siteConfig.siteName}</title>
</svelte:head>

<p class="mb-2 flex flex-row items-center justify-start text-primary-500">
    <Icon icon="tabler:plus" height="24" class="mr-2" />
    Create Package
</p>

<div class="card variant-soft-secondary w-full p-4">
    <p class="mb-2 flex flex-row items-center justify-start text-primary-500">
        <Icon icon="tabler:eye" height="24" class="mr-2" />
        Display Name
    </p>

    <input
        type="text"
        placeholder="Example: My Package"
        class="input rounded-md"
        oninput={updateSlug}
        bind:value={name}
    />
</div>

<div class="card variant-soft-secondary w-full p-4">
    <p class="mb-2 flex flex-row items-center justify-start text-primary-500">
        <Icon icon="tabler:link" height="24" class="mr-2" />
        Slug
    </p>

    <input
        type="text"
        placeholder="Example: my-package"
        class="input rounded-md"
        bind:value={slug}
    />

    {#if slugError}
        <p class="ml-1 mt-2 text-error-500">Project already exists!</p>
    {/if}
</div>

<div class="card variant-soft-secondary w-full p-4">
    <p class="mb-2 flex flex-row items-center justify-start text-primary-500">
        <Icon icon="tabler:info-circle-filled" height="24" class="mr-2" />
        Summary
    </p>

    <input
        type="text"
        placeholder="A short description of your project"
        class="input rounded-md"
        bind:value={description}
    />
</div>

<div class="card variant-soft-secondary w-full p-4">
    <p class="mb-2 flex flex-row items-center justify-start text-primary-500">
        <Icon icon="tabler:code" height="24" class="mr-2" />
        Source Code
    </p>

    <input
        type="text"
        placeholder="Example: https://github.com/example/example"
        class="input rounded-md"
        bind:value={repo}
    />
</div>

<div class="card variant-soft-secondary w-full p-4">
    <p class="mb-2 flex flex-row items-center justify-start text-primary-500">
        <Icon icon="tabler:exclamation-circle" height="24" class="mr-2" />
        Issue Tracker
    </p>

    <input
        type="text"
        placeholder="Example: https://github.com/example/example/issues"
        class="input rounded-md"
        bind:value={issues}
    />
</div>

<div class="card variant-soft-secondary w-full p-4">
    <p class="mb-2 flex flex-row items-center justify-start text-primary-500">
        <Icon icon="tabler:world" height="24" class="mr-2" />
        Wiki
    </p>

    <input
        type="text"
        placeholder="Example: https://github.com/example/example/wiki"
        class="input rounded-md"
        bind:value={wiki}
    />
</div>

<div class="card variant-soft-secondary w-full p-4">
    <p class="mb-2 flex flex-row items-center justify-start text-primary-500">
        <Icon icon="tabler:license" height="24" class="mr-2" />
        License
    </p>

    <input
        type="text"
        name="autocomplete-license"
        placeholder="Choose a license (or type your own)"
        class="autocomplete input rounded-md"
        bind:value={license}
        use:popup={licensesPopup}
    />

    <div
        data-popup="licensesAutocomplete"
        class="h-[50%] w-[40%] overflow-scroll rounded-lg bg-secondary-700 p-2"
    >
        <Autocomplete bind:input={license} options={allLicenses} on:selection={onLicenseSelect} />
    </div>
</div>

<div class="card variant-soft-secondary w-full p-4">
    <p class="mb-2 flex flex-row items-center justify-start text-primary-500">
        <Icon icon="tabler:eye" height="24" class="mr-2" />
        Visibility (Coming Soon)
    </p>

    <select
        class="select variant-ghost-primary cursor-pointer !outline-none"
        bind:value={visibility}
        disabled
    >
        <option value="Public">Public</option>
        <option value="Private">Private</option>
        <option value="Unlisted">Unlisted</option>
    </select>
</div>

<div class="card variant-soft-secondary w-full p-4">
    <p class="mb-2 flex flex-row items-center justify-start text-primary-500">
        <Icon icon="tabler:file-description" height="24" class="mr-2" />
        Description
    </p>

    <div class="card variant-soft-secondary w-full p-4">
        <MarkdownEditor carta={editor} bind:value={readme} mode="tabs" />
    </div>
</div>

<div class="flex flex-row items-center justify-start gap-2">
    <button
        type="button"
        class="variant-filled-primary btn mt-2 flex flex-row items-center justify-center rounded-lg transition-all hover:variant-ghost-primary hover:text-token"
        onclick={save}
    >
        <Icon icon="tabler:plus" height="24" class="mr-2" />
        Create
    </button>

    <button
        type="button"
        class="variant-ghost-secondary btn mt-2 flex flex-row items-center justify-center rounded-lg transition-all hover:variant-filled-secondary"
        onclick={() => goto("/")}
    >
        <Icon icon="tabler:trash" height="24" class="mr-2" />
        Cancel
    </button>
</div>
