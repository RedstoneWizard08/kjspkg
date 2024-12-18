<script lang="ts">
    import { _ } from "svelte-i18n";
    import { page } from "$app/stores";
    import { onMount } from "svelte";
    import { getPackageGallery, deleteGalleryImage } from "$api";
    import { currentPackage } from "$lib/stores";
    import Icon from "@iconify/svelte";
    import type { PublicGalleryImage } from "$lib/types/gallery";
    import { getModalStore } from "@skeletonlabs/skeleton";

    const id = $derived($page.params.id);
    const modals = getModalStore();

    let images = $state<PublicGalleryImage[]>([]);

    onMount(async () => {
        if (!$currentPackage) return;

        images = (await getPackageGallery(id)) ?? [];
    });

    modals.subscribe(async () => {
        images = (await getPackageGallery(id)) ?? [];
    });

    const deleteImage = (img: number) => {
        return async (ev: Event) => {
            ev.preventDefault();
            ev.stopPropagation();

            modals.trigger({
                type: "component",
                component: "confirmDeleteImage",
                meta: { imageId: img },
            });
        };
    };
</script>

<p class="mb-2 flex flex-row items-center justify-start text-primary-500">
    <Icon icon="tabler:library-photo" height="24" class="mr-2" />
    Gallery
</p>

{#if images.length >= 1}
    <div
        class="card variant-glass-surface grid w-full grid-cols-[1fr_1fr] gap-2 p-4 md:grid-cols-[1fr_1fr_1fr]"
    >
        {#each images as img}
            <a
                href="/p/{id}/edit/gallery/edit/{img.id}"
                class="card flex h-full w-full cursor-pointer flex-col items-center justify-start gap-y-4 p-2 transition-all hover:variant-soft-primary"
            >
                <p class="w-[90%] text-left text-lg font-bold text-ellipsis overflow-clip">{img.name}</p>

                <img
                    src={img.url}
                    alt={img.name}
                    class="aspect-square w-[90%] rounded-lg object-cover"
                />

                <div class="flex w-[90%] flex-row items-center justify-end gap-2">
                    <!-- This has no onclick handler because it just passes through to the underlying link -->
                    <button
                        type="button"
                        class="variant-glass-primary btn btn-sm transition-all hover:variant-filled-primary"
                    >
                        <Icon icon="tabler:pencil" height="24" />
                    </button>

                    <button
                        type="button"
                        class="variant-glass-error btn btn-sm transition-all hover:variant-filled-error"
                        onclick={deleteImage(img.id)}
                    >
                        <Icon icon="tabler:trash" height="24" />
                    </button>
                </div>
            </a>
        {/each}
    </div>
{:else}
    <div
        class="card variant-glass-surface flex w-full flex-row items-center justify-center p-4 py-16"
    >
        No images found!
    </div>
{/if}

<div class="card variant-glass-surface w-full space-y-2 p-4">
    <p class="mb-4 flex flex-row items-center justify-start text-primary-500">
        <Icon icon="tabler:upload" height="24" class="mr-2" />
        Upload Image
    </p>

    <a
        href="/p/{id}/edit/gallery/create"
        class="variant-ghost-secondary btn flex w-full flex-row items-center justify-center rounded-lg transition-all hover:variant-soft-primary"
    >
        <Icon icon="tabler:upload" height="24" class="mr-2" />
        Upload Image
    </a>
</div>
