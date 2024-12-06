<script lang="ts">
    import { _ } from "svelte-i18n";
    import { page } from "$app/stores";
    import { onMount } from "svelte";
    import { getPackageVersions } from "$api";
    import { currentPackage } from "$lib/stores";
    import Icon from "@iconify/svelte";
    import type { PackageVersion } from "$lib/types";
    import EditVersion from "$components/ui/edit/EditVersion.svelte";
    import { getModalStore } from "@skeletonlabs/skeleton";

    const id = $derived($page.params.id);
    const modals = getModalStore();

    let vers = $state<PackageVersion[]>([]);
    let loading = $state(true);

    onMount(async () => {
        if (!$currentPackage) return;

        vers = (await getPackageVersions(id)) ?? [];
        loading = false;
    });

    modals.subscribe(async () => {
        vers = (await getPackageVersions(id)) ?? [];
    });
</script>

<p class="mb-2 flex flex-row items-center justify-start text-primary-500">
    <Icon icon="tabler:versions" height="24" class="mr-2" />
    Manage Versions
</p>

{#if loading}
    <div class="card variant-soft-secondary w-full space-y-2 p-4">
        <div class="flex flex-row items-center justify-center">
            <p class="flex flex-row items-center justify-center">
                <Icon icon="tabler:loader-2" height="24" class="mr-2 animate-spin" />
                Loading...
            </p>
        </div>
    </div>
{:else if vers.length >= 1}
    <div class="card variant-soft-secondary w-full space-y-2 p-4">
        {#each vers as version}
            <EditVersion {version} pkg={id} />
        {/each}
    </div>
{:else}
    <div
        class="card variant-soft-secondary flex w-full flex-row items-center justify-center p-4 py-16"
    >
        No images found!
    </div>
{/if}

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
