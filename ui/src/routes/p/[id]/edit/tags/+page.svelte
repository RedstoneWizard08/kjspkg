<script lang="ts">
    import { _ } from "svelte-i18n";
    import { page } from "$app/stores";
    import { onMount } from "svelte";
    import { getPackage } from "$api";
    import { currentPackage, editSaving } from "$lib/stores";
    import Icon from "@iconify/svelte";

    const id = $derived($page.params.id);

    onMount(() => {
        if (!$currentPackage) return;

        // Fields go here
    });

    const save = async () => {
        $editSaving = true;

        // Update code goes here

        $currentPackage = await getPackage(id);

        // Field update code goes here

        $editSaving = false;
    };
</script>

<p class="h3">Coming Soon!</p>

<!-- <button
    type="button"
    class="btn variant-filled-primary mt-2 flex flex-row items-center justify-center rounded-lg"
    onclick={save}
>
    <Icon icon="tabler:device-floppy" height="24" class="mr-2" />
    Save
</button> -->
