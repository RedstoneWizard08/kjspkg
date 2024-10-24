<script lang="ts">
    import { goto } from "$app/navigation";
    import { base } from "$app/paths";
    import AuthRequired from "$components/auth/AuthRequired.svelte";
    import { user, currentSearchStore } from "$lib/stores";

    let renderErrPage = false;

    setTimeout(() => {
        renderErrPage = true;
    }, 100);

    user.subscribe((user) => {
        if (user) {
            window.location.href = `${base}/s/?q=${user.username}`;
        }
    });
</script>

{#if renderErrPage}
    <AuthRequired />
{/if}
