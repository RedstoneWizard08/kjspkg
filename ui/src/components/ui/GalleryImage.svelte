<script lang="ts">
    import type { PublicGalleryImage } from "$lib/types/gallery";
    import { getModalStore } from "@skeletonlabs/skeleton";

    const { img }: { img: PublicGalleryImage } = $props();
    const modals = getModalStore();

    const openViewModal = () => {
        modals.trigger({
            type: "component",
            component: "imageView",
            meta: { img },
        });
    };
</script>

<button
    type="button"
    onclick={openViewModal}
    class="card flex h-full w-full cursor-pointer flex-col items-center justify-start gap-y-4 p-2 transition-all hover:variant-soft-primary"
>
    <p class="w-[90%] text-left text-lg font-bold text-ellipsis overflow-clip">{img.name}</p>

    <img src={img.url} alt={img.name} class="aspect-square w-[90%] rounded-lg object-cover" />
</button>
