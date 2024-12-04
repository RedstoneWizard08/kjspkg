<script lang="ts">
    import { _ } from "svelte-i18n";
    import { page } from "$app/stores";
    import type { PackageVersion } from "$lib/types";
    import { markdown } from "$lib/utils";
    import { fly } from "svelte/transition";
    import { onMount } from "svelte";
    import { getPackageVersion } from "$api";
    import { currentPackage } from "$lib/stores";
    import { siteConfig } from "$lib/config";

    const id = $derived($page.params.id);
    const ver = $derived($page.params.ver);

    let repo = $state("");
    let issues = $state("");
    let wiki = $state("");

    let version: PackageVersion | undefined = $state(undefined);

    let name = $state("");
    let changelog = $state<string | undefined>(undefined);

    onMount(async () => {
        version = await getPackageVersion(id, ver);

        if (!$currentPackage || !version) return;

        name = version.name;
        changelog = version.changelog;
        repo = $currentPackage.source ?? "";
        issues = $currentPackage.issues ?? "";
        wiki = $currentPackage.wiki ?? "";
    });
</script>

<svelte:head>
    <title>{version?.name ?? $_("site.loading")} - {siteConfig.siteName}</title>
</svelte:head>

<section class="card h-fit w-full p-4" in:fly={{ y: 20 }}>
    <dt class="text-sm opacity-50">
        {$_("package.version.changelog")}
    </dt>

    <dd
        class="style-markdown flex select-text flex-col items-start *:select-text"
        in:fly={{ y: 20 }}
    >
        {@html markdown(changelog ?? "")}
    </dd>
</section>
