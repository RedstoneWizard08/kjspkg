<script lang="ts">
    import { goto } from "$app/navigation";
    import { base } from "$app/paths";
    import { page } from "$app/stores";
    import { getToastStore } from "@skeletonlabs/skeleton";
    import { onMount } from "svelte";

    const toastStore = getToastStore();

    onMount(() => {
        toastStore.trigger({
            message: `${$page.error?.message}`,
            hideDismiss: true,
            timeout: 5000,
            background: "variant-filled-error",
        });

        setTimeout(() => {
            goto(base + "/");
        }, 5000);
    });
</script>

<!-- TODO: Better error page -->
