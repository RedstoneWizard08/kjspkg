<script lang="ts">
    import { _ } from "svelte-i18n";
    import type { PackageVersion } from "$lib/types";
    import { formatDate } from "$lib/utils";
    import TablerIcon from "$components/icons/TablerIcon.svelte";
    import { deleteVersion } from "$api";
    import { getToastStore } from "@skeletonlabs/skeleton";

    interface Props {
        version: PackageVersion;
        pkg: string;
        editing: boolean;
        onDelete: () => void | Promise<void>;
    }

    const { version, pkg, editing, onDelete }: Props = $props();

    let loading = $state(false);

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
                message: $_("errors.delete_version"),
            });

            loading = false;
        }

        await onDelete();

        loading = false;
    };
</script>

<a
    href="/p/{pkg}/v/{version.id}"
    class="flex w-full items-center gap-2 p-2 text-left rounded-container-token hover:variant-soft-primary"
>
    <TablerIcon name="download" />
    <span class="ml-1 flex-auto">
        <dt class="select-text font-bold">{version.name}</dt>
        <dd class="text-sm opacity-50">{formatDate(new Date(version.created_at))}</dd>
    </span>

    {#if editing}
        <button
            class="btn-ghost variant-ringed-error btn btn-sm transition-all hover:variant-filled-error"
            type="button"
            onclick={handleDelete}
            disabled={loading}
        >
            <TablerIcon name="trash" />
        </button>
    {/if}
</a>
