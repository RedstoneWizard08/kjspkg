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

<a
    href="/p/{pkg}/v/{version.id}"
    class="flex w-full items-center gap-2 p-2 text-left rounded-container-token hover:variant-soft-primary"
>
    <IconDownload />
    <span class="ml-1 flex-auto">
        <dt class="select-text font-bold">{version.name}</dt>
        <dd class="text-sm opacity-50">{formatDate(new Date(version.created_at))}</dd>
    </span>
</a>
