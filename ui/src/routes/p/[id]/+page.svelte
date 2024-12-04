<script lang="ts">
    import { beforeNavigate } from "$app/navigation";
    import { _ } from "svelte-i18n";
    import { page } from "$app/stores";
    import type { PackageVersion } from "$lib/types";
    import { markdown } from "$lib/utils";
    import { fly } from "svelte/transition";
    import { onMount } from "svelte";
    import { getPackageVersions } from "$api";
    import Version from "$components/ui/Version.svelte";
    import { currentPackage } from "$lib/stores";

    const id = $derived($page.params.id);

    let versions: PackageVersion[] = $state([]);

    let readme = $state("");

    const sortedVersions = $derived(
        [...versions]
            .sort((a, b) => new Date(a.created_at).getTime() - new Date(b.created_at).getTime())
            .reverse(),
    );

    onMount(async () => {
        if (!$currentPackage) return;

        versions = (await getPackageVersions(id)) ?? [];

        readme = $currentPackage.readme;
    });

    beforeNavigate(() => {
        $currentPackage = undefined;
    });
</script>

<div class="grid w-full grid-cols-1 gap-2 lg:grid-cols-2">
    <section class="card h-fit p-4 lg:col-span-2" in:fly={{ y: 20 }}>
        <dt class="text-sm opacity-50">
            {$_("package.readme_file")}
        </dt>
        <dd
            class="style-markdown flex select-text flex-col items-start *:select-text"
            in:fly={{ y: 20 }}
        >
            {@html markdown(readme)}
        </dd>
    </section>

    {#if sortedVersions.length > 0}
        <div class="card h-fit space-y-2 p-4 lg:col-span-2" in:fly|global={{ y: 20 }}>
            <dt class="text-sm opacity-50">{$_("package.versions")}</dt>

            <dd class="flex w-full gap-1">
                <dl class="list-dl w-full">
                    {#each sortedVersions as version}
                        <Version {version} pkg={$currentPackage!.slug} />
                    {/each}
                </dl>
            </dd>
        </div>
    {/if}
</div>
