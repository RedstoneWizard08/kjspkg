<script lang="ts">
    import type { PublicGalleryImage } from "$lib/types/gallery";
    import { markdown } from "$lib/utils";
    import { getModalStore, getToastStore } from "@skeletonlabs/skeleton";
    import { onMount } from "svelte";

    const modals = getModalStore();
    const toasts = getToastStore();
    let img: PublicGalleryImage;

    onMount(() => {
        if (!$modals[0].meta || !("img" in $modals[0].meta)) {
            toasts.trigger({
                message: `Error: Missing property 'img' in $modals[0].meta!`,
                hideDismiss: true,
                timeout: 5000,
                background: "variant-filled-error",
            });

            modals.close();

            return;
        }

        img = $modals[0].meta.img;
    });
</script>

{#if $modals[0] && img}
    <div class="w-modal-slim relative rounded-lg bg-surface-500 p-8 shadow-xl">
        <header class="text-2xl font-bold">{img.name}</header>

        <img src={img.url} alt={img.name} class="my-4" />

        <div class="style-markdown flex select-text flex-col items-start *:select-text">
            {@html markdown(img.description ?? "")}
        </div>

        <footer class="modal-footer mt-4 flex flex-row items-center">
            <button
                class="variant-filled-secondary btn mr-2 !outline-none hover:variant-ghost-primary"
                onclick={() => modals.close()}>Close</button
            >
        </footer>
    </div>
{/if}
