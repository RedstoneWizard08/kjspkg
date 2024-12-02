<script lang="ts">
    import { _ } from "svelte-i18n";
    import type { PackageVersion } from "$lib/types";
    import { formatDate } from "$lib/utils";
    import { deleteVersion } from "$api";
    import { getToastStore } from "@skeletonlabs/skeleton";
    import { downloadFile } from "$lib/download";
    import Icon from "@iconify/svelte";

    interface Props {
        version: PackageVersion;
        pkg: string;
        editing: boolean;
        onDelete: () => void | Promise<void>;
    }

    const { version, pkg, editing, onDelete }: Props = $props();

    let loading = $state(false);
    let downloading = $state(false);
    let done = $state(false);

    const handleDelete = async (ev: Event) => {
        ev.preventDefault();
        ev.stopPropagation();

        if (loading) {
            return;
        }

        loading = true;

        try {
            if (!(await deleteVersion(pkg, version.id))) {
                throw new Error("Failed to delete version");
            }
        } catch (e) {
            getToastStore().trigger({
                background: "variant-filled-error",
                message: $_("errors.delete.version"),
            });

            loading = false;
        }

        await onDelete();

        loading = false;
    };

    let doneTimeout: number | undefined;

    const directDownload = async (ev: Event) => {
        ev.preventDefault();
        ev.stopPropagation();

        downloading = true;

        await downloadFile(
            `/api/v1/packages/${pkg}/versions/${version.id}/download`,
            version.file_name,
        );

        downloading = false;
        done = true;

        if (doneTimeout) clearTimeout(doneTimeout);

        doneTimeout = setTimeout(() => {
            done = false;
        }, 1000) as any;
    };
</script>

<a
    href="/p/{pkg}/v/{version.id}"
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

    {#if editing}
        <button
            class="btn-ghost variant-glass-error btn btn-sm transition-all hover:variant-filled-error"
            type="button"
            onclick={handleDelete}
            disabled={loading}
        >
            <Icon icon="tabler:trash" height="24" />
        </button>
    {/if}
</a>
