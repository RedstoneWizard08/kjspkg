<script lang="ts">
    import { _ } from "svelte-i18n";
    import type { PackageVersion } from "$lib/types";
    import { formatDate } from "$lib/utils";
    import { getPackage } from "$api";
    import { getModalStore } from "@skeletonlabs/skeleton";
    import { downloadFile } from "$lib/download";
    import Icon from "@iconify/svelte";

    interface Props {
        version: PackageVersion;
        pkg: string;
    }

    const { version, pkg }: Props = $props();
    const modals = getModalStore();

    let downloading = $state(false);
    let done = $state(false);

    const handleDelete = async (ev: Event) => {
        ev.preventDefault();
        ev.stopPropagation();

        modals.trigger({
            type: "component",
            component: "confirmDeleteVersion",
            meta: { versionId: version.id },
        });
    };

    let doneTimeout: number | undefined;

    const directDownload = async (ev: Event) => {
        ev.preventDefault();
        ev.stopPropagation();

        downloading = true;

        const pkgData = await getPackage(pkg);
        const fileName = `${pkgData?.slug}_${version.version_number}.mhpkg`;

        await downloadFile(`/api/v1/packages/${pkg}/versions/${version.id}/download`, fileName);

        downloading = false;
        done = true;

        if (doneTimeout) clearTimeout(doneTimeout);

        doneTimeout = setTimeout(() => {
            done = false;
        }, 1000) as any;
    };
</script>

<a
    href="/p/{pkg}/edit/versions/edit/{version.id}"
    class="flex w-full items-center gap-2 p-2 text-left transition-all rounded-container-token hover:variant-soft-primary"
>
    <button
        type="button"
        class="variant-filled-secondary btn p-2 transition-all hover:variant-outline-primary"
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

    <span class="ml-1 flex-auto">
        <dt class="select-text font-bold">{version.name}</dt>
        <dd class="text-sm opacity-50">{formatDate(new Date(version.created_at))}</dd>
    </span>

    <!-- This has no onclick handler because it just passes through to the underlying link -->
    <button
        class="btn-ghost variant-glass-primary btn btn-sm transition-all hover:variant-filled-primary"
        type="button"
    >
        <Icon icon="tabler:pencil" height="24" />
    </button>

    <button
        class="btn-ghost variant-glass-error btn btn-sm transition-all hover:variant-filled-error"
        type="button"
        onclick={handleDelete}
    >
        <Icon icon="tabler:trash" height="24" />
    </button>
</a>
