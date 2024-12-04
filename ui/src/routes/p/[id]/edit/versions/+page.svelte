<script lang="ts">
    import { _ } from "svelte-i18n";
    import { page } from "$app/stores";
    import { onMount } from "svelte";
    import { getPackage, getPackageVersions } from "$api";
    import { currentPackage, editSaving } from "$lib/stores";
    import Icon from "@iconify/svelte";
    import type { PackageVersion } from "$lib/types";
    import EditVersion from "$components/ui/edit/EditVersion.svelte";

    const id = $derived($page.params.id);

    let vers = $state<PackageVersion[]>([]);
    let loading = $state(true);

    onMount(async () => {
        if (!$currentPackage) return;

        vers = (await getPackageVersions(id)) ?? [];

        loading = false;
    });

    const save = async () => {
        $editSaving = true;

        // Update code goes here

        $currentPackage = await getPackage(id);

        // Field update code goes here

        $editSaving = false;
    };
</script>

<p class="mb-2 flex flex-row items-center justify-start text-primary-500">
    <Icon icon="tabler:versions" height="24" class="mr-2" />
    Manage Versions
</p>

<div class="card variant-soft-secondary w-full space-y-2 p-4">
    <p class="mb-4 flex flex-row items-center justify-start text-primary-500">
        <Icon icon="tabler:versions" height="24" class="mr-2" />
        Project Versions
    </p>

    {#if loading}
        <div class="flex flex-row items-center justify-center">
            <p class="flex flex-row items-center justify-center">
                <Icon icon="tabler:loader-2" height="24" class="mr-2 animate-spin" />
                Loading...
            </p>
        </div>
    {:else}
        {#each vers as version}
            <EditVersion {version} pkg={id} />
        {/each}
    {/if}
</div>

<div class="card variant-soft-secondary w-full space-y-2 p-4">
    <p class="mb-4 flex flex-row items-center justify-start text-primary-500">
        <Icon icon="tabler:upload" height="24" class="mr-2" />
        Upload Version
    </p>

    <a
        href="/p/{id}/edit/versions/create"
        class="variant-ghost-secondary btn flex w-full flex-row items-center justify-center rounded-lg transition-all hover:variant-soft-primary"
    >
        <Icon icon="tabler:upload" height="24" class="mr-2" />
        Upload Version
    </a>
</div>
