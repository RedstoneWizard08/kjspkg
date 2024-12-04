<script lang="ts">
    import { _ } from "svelte-i18n";
    import { page } from "$app/stores";
    import { onMount } from "svelte";
    import { getPackage, updatePackage } from "$api";
    import { currentPackage, editSaving } from "$lib/stores";
    import Icon from "@iconify/svelte";
    import { Carta, MarkdownEditor } from "carta-md";

    const id = $derived($page.params.id);
    const editor = new Carta();

    let readme = $state("");

    onMount(() => {
        if (!$currentPackage) return;

        readme = $currentPackage.readme;
    });

    const save = async () => {
        $editSaving = true;

        await updatePackage(id, {
            readme,
        });

        $currentPackage = await getPackage(id);

        readme = $currentPackage?.readme ?? readme;

        $editSaving = false;
    };
</script>

<p class="mb-2 flex flex-row items-center justify-start text-primary-500">
    <Icon icon="tabler:file-description" height="24" class="mr-2" />
    Edit Description
</p>

<div class="card variant-soft-secondary w-full p-4">
    <MarkdownEditor carta={editor} bind:value={readme} mode="tabs" />
</div>

<button
    type="button"
    class="variant-filled-primary btn mt-2 flex flex-row items-center justify-center rounded-lg"
    onclick={save}
>
    <Icon icon="tabler:device-floppy" height="24" class="mr-2" />
    Save
</button>
