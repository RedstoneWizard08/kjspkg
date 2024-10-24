<script lang="ts">
    import { _ } from "svelte-i18n";
    import { IconDownload } from "@tabler/icons-svelte";
    import type { PackageVersion } from "$lib/types";
    import { formatDate } from "$lib/utils";
    import { contextMenu, type ContextMenuItem } from "$lib/contextMenu";
    import { getToastStore } from "@skeletonlabs/skeleton";
    import { copyText } from "$lib/clipboard";

    const { version, pkg }: { version: PackageVersion; pkg: string } = $props();
    const toasts = getToastStore();
</script>

<button
    class="rounded-container-token hover:variant-soft-primary flex w-full items-center gap-2 p-2 text-left"
    use:contextMenu={{
        initiator: "left",
        items: [
            { type: "SEPARATOR", header: $_("package.version.action") },
            {
                type: "ITEM",
                label: `kjspkg install ${pkg}@${version.version_number}`,
                action: async () => {
                    await copyText(`kjspkg install ${pkg}@${version.version_number}`, toasts);
                },
            },
            {
                type: "ITEM",
                label: `kjspkg update ${pkg}@${version.version_number}`,
                action: async () => {
                    await copyText(`kjspkg update ${pkg}@${version.version_number}`, toasts);
                },
            },
            {
                type: "ITEM",
                label: `kjspkg remove ${pkg}@${version.version_number}`,
                action: async () => {
                    await copyText(`kjspkg remove ${pkg}@${version.version_number}`, toasts);
                },
            },
        ],
    }}
>
    <IconDownload />
    <span class="flex-auto ml-1">
        <dt class="select-text font-bold">{version.name}</dt>
        <dd class="text-sm opacity-50">{formatDate(new Date(version.created_at))}</dd>
    </span>
</button>
