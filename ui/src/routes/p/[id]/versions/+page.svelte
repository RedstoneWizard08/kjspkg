<script lang="ts">
    import { _ } from "svelte-i18n";
    import { page } from "$app/stores";
    import type { PackageVersion } from "$lib/types";
    import { fly } from "svelte/transition";
    import { onMount } from "svelte";
    import { getPackageVersions } from "$api";
    import Version from "$components/ui/Version.svelte";
    import { currentPackage } from "$lib/stores";

    const id = $derived($page.params.id);

    let versions: PackageVersion[] = $state([]);

    const sortedVersions = $derived(
        [...versions]
            .sort((a, b) => new Date(a.created_at).getTime() - new Date(b.created_at).getTime())
            .reverse(),
    );

    onMount(async () => {
        if (!$currentPackage) return;

        versions = (await getPackageVersions(id)) ?? [];
    });
</script>

{#if sortedVersions.length > 0}
    <div class="card h-fit w-full space-y-2 p-4">
        <dt class="text-sm opacity-50">{$_("package.versions")}</dt>

        <dd class="flex w-full gap-1">
            <dl class="list-dl w-full">
                {#each sortedVersions as version}
                    <Version {version} pkg={$currentPackage!.slug} />
                {/each}
            </dl>
        </dd>
    </div>
{:else}
    <div class="card flex h-fit w-full flex-col space-y-2 p-4">
        <dt class="text-sm opacity-50">{$_("package.versions")}</dt>
        <span class="w-full py-8 text-center">No versions found!</span>
    </div>
{/if}
