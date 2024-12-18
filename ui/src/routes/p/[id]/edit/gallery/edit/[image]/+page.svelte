<script lang="ts">
    import { _ } from "svelte-i18n";
    import { page } from "$app/stores";
    import { onMount } from "svelte";
    import { getGalleryImage, getPackage, updateGalleryImage } from "$api";
    import { currentPackage, editSaving } from "$lib/stores";
    import Icon from "@iconify/svelte";
    import { getModalStore, popup, type PopupSettings } from "@skeletonlabs/skeleton";
    import { Carta, MarkdownEditor } from "carta-md";
    import type { PublicGalleryImage } from "$lib/types/gallery";

    const id = $derived($page.params.id);
    const imgId = $derived($page.params.image);
    const editor = new Carta();
    const modals = getModalStore();

    let img = $state<PublicGalleryImage | undefined>(undefined);
    let name = $state("");
    let description = $state("");
    let ordering = $state(-1);

    onMount(async () => {
        if (!$currentPackage || !imgId) return;

        img = await getGalleryImage(id, imgId);

        name = img?.name ?? "";
        description = img?.description ?? "";
        ordering = img?.ordering ?? -1;
    });

    const save = async () => {
        $editSaving = true;

        await updateGalleryImage(id, imgId, {
            name,
            ordering,
            description: description == "" ? undefined : description,
        });

        $currentPackage = await getPackage(id);
        img = await getGalleryImage(id, imgId);

        name = img?.name ?? "";
        description = img?.description ?? "";
        ordering = img?.ordering ?? -1;

        $editSaving = false;
    };

    const deleteImage = async () => {
        modals.trigger({
            type: "component",
            component: "confirmDeleteImage",
            meta: { imageId: imgId },
        });
    };

    const orderingInfoPopup: PopupSettings = {
        event: "hover",
        target: "orderingInfoPopup",
        placement: "bottom",
    };
</script>

<p class="mb-2 flex flex-row items-center justify-start text-primary-500">
    <Icon icon="tabler:pencil" height="24" class="mr-2" />
    Edit Gallery Image
</p>

<div class="card variant-glass-surface w-full p-4">
    <p class="mb-2 flex flex-row items-center justify-start text-primary-500">
        <Icon icon="tabler:label" height="24" class="mr-2" />
        Name
    </p>

    <input type="text" placeholder="Example: My Image" class="input rounded-md" bind:value={name} />
</div>

<div class="card variant-glass-surface w-full p-4">
    <div class="flex w-full flex-row items-center justify-between">
        <p class="mb-2 flex flex-row items-center justify-start text-primary-500">
            <Icon icon="tabler:arrows-sort" height="24" class="mr-2" />
            Ordering
        </p>

        <div use:popup={orderingInfoPopup} class="flex flex-row items-center justify-end">
            <Icon
                icon="tabler:info-circle"
                height="24"
                class="pointer-events-none mr-2 text-success-500"
            />
        </div>

        <div class="z-20 rounded-lg bg-secondary-700 p-4" data-popup="orderingInfoPopup">
            A higher number will be displayed first, and a lower number last.
        </div>
    </div>

    <input type="number" placeholder="Example: -1" class="input rounded-md" bind:value={ordering} />
</div>

<div class="card variant-glass-surface w-full p-4">
    <p class="mb-2 flex flex-row items-center justify-start text-primary-500">
        <Icon icon="tabler:file-description" height="24" class="mr-2" />
        Edit Description (Optional)
    </p>

    <MarkdownEditor carta={editor} bind:value={description} mode="tabs" />
</div>

<div class="flex flex-row items-center justify-start gap-2">
    <button
        type="button"
        class="variant-filled-primary btn mt-2 flex flex-row items-center justify-center rounded-lg transition-all hover:variant-ghost-primary hover:text-token"
        onclick={save}
    >
        <Icon icon="tabler:device-floppy" height="24" class="mr-2" />
        Save
    </button>

    <button
        type="button"
        class="variant-filled-error btn mt-2 flex flex-row items-center justify-center rounded-lg transition-all hover:variant-ghost-error"
        onclick={deleteImage}
    >
        <Icon icon="tabler:trash" height="24" class="mr-2" />
        Delete Image
    </button>
</div>
