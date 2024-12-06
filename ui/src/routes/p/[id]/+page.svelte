<script lang="ts">
    import { _ } from "svelte-i18n";
    import { markdown } from "$lib/utils";
    import { fly } from "svelte/transition";
    import { onMount } from "svelte";
    import { currentPackage } from "$lib/stores";

    let readme = $state("");

    onMount(async () => {
        if (!$currentPackage) return;

        readme = $currentPackage.readme;
    });
</script>

<section class="card h-fit w-full p-4">
    <dt class="text-sm opacity-50">Description</dt>
    <dd class="style-markdown flex select-text flex-col items-start *:select-text">
        {@html markdown(readme)}
    </dd>
</section>
