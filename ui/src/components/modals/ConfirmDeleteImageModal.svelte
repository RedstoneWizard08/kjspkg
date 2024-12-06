<script lang="ts">
    import { _ } from "svelte-i18n";
    import { getModalStore, getToastStore } from "@skeletonlabs/skeleton";
    import { deleteGalleryImage } from "$api";
    import { currentPackage } from "$lib/stores";
    import { goto } from "$app/navigation";

    const modals = getModalStore();
    const toasts = getToastStore();
    let loading = $state(false);

    const confirmDelete = async () => {
        loading = true;

        if (!$currentPackage) {
            toasts.trigger({
                message: `Internal error: $currentPackage is undefined!`,
                hideDismiss: true,
                timeout: 5000,
                background: "variant-filled-error",
            });

            loading = false;

            return;
        }

        if (!$modals[0].meta || !("imageId" in $modals[0].meta)) {
            toasts.trigger({
                message: `Internal error: $modals[0].meta.imageId is undefined!`,
                hideDismiss: true,
                timeout: 5000,
                background: "variant-filled-error",
            });

            loading = false;

            return;
        }

        const { imageId } = $modals[0].meta;

        await deleteGalleryImage($currentPackage.id, imageId);

        loading = false;
        modals.close();
        goto(`/p/${$currentPackage.id}/edit/gallery`, { invalidateAll: true });
    };
</script>

{#if $modals[0]}
    <div class="w-modal-slim relative rounded-lg bg-surface-500 p-8 shadow-xl">
        <header class="text-2xl font-bold">Confirm Deletion</header>

        <p>Are you sure you want to delete this gallery image?</p>

        <footer class="modal-footer mt-4 flex flex-row items-center">
            <button
                class="variant-filled-error btn mr-2 !outline-none transition-all hover:variant-ghost-error"
                disabled={loading}
                onclick={confirmDelete}>Delete</button
            >

            <button
                class="variant-filled-secondary btn mr-2 !outline-none transition-all hover:variant-ghost-primary"
                disabled={loading}
                onclick={() => modals.close()}>{$_("action.cancel")}</button
            >
        </footer>
    </div>
{/if}
