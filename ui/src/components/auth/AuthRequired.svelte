<script lang="ts">
    import { _ } from "svelte-i18n";
    import { page } from "$app/stores";
    import { fly } from "svelte/transition";
    import TablerIcon from "$components/icons/TablerIcon.svelte";
</script>

<svelte:head>
    <title>Authentication Required - KJSPKG Lookup</title>
</svelte:head>

<div class="flex flex-1 flex-col items-center justify-center" transition:fly={{ y: 40 }}>
    <TablerIcon
        name="alert-triangle"
        class="h-36 w-auto text-[rgb(var(--color-primary-500))] lg:h-60"
    />
    <h1 class="h3 w-64 text-center lg:w-96">{$_("author.auth_required")}</h1>

    <a
        class="card mt-4 flex gap-2 px-5 py-3 hover:variant-soft-primary"
        href="/api/v1/auth/github/login?redirect_uri={$page.url}"
    >
        <TablerIcon name="brand-github" />
        {$_("author.auth_required.login")}
    </a>
</div>
