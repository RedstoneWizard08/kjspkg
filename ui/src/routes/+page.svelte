<script lang="ts">
    import { goto } from "$app/navigation";
    import { base } from "$app/paths";
    import { page } from "$app/stores";
    import { userPreferencesStore } from "$lib/stores";
    import { onMount } from "svelte";

    onMount(() => {
        if ($page.url.hash != "") {
            goto(base + `/p/${$page.url.hash.substring(1)}`);
            return;
        }

        if (!$userPreferencesStore.alreadyVisited) {
            userPreferencesStore.update((prev) => ({ ...prev, alreadyVisited: true }));
            goto(base + "/home");
            return;
        }

        goto(base + "/s");
    });
</script>
