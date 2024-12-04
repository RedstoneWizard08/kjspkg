<script lang="ts">
    import { _ } from "svelte-i18n";
    import { getModalStore, getToastStore } from "@skeletonlabs/skeleton";
    import { deletePackage } from "$api";
    import { currentPackage, forceUpdatePackagesStore } from "$lib/stores";
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

        await deletePackage($currentPackage.id);
        await forceUpdatePackagesStore();

        loading = false;
        modals.close();
        goto("/s");
    };
</script>

{#if $modals[0]}
    <div class="w-modal-slim relative rounded-lg bg-surface-500 p-8 shadow-xl">
        <header class="text-2xl font-bold">Confirm Delete</header>

        <p>Are you sure you want to delete your project, {$currentPackage?.name}?</p>

        <footer class="modal-footer mt-4 flex flex-row items-center">
            <button
                class="variant-filled-error btn mr-2 !outline-none hover:variant-ghost-error"
                disabled={loading}
                onclick={confirmDelete}>Delete</button
            >

            <button
                class="variant-filled-secondary btn mr-2 !outline-none hover:variant-ghost-primary"
                disabled={loading}
                onclick={() => modals.close()}>{$_("action.cancel")}</button
            >
        </footer>
    </div>
{/if}
