<script lang="ts">
    import { isLoggedIn } from "$api";
    import { base } from "$app/paths";
    import { page } from "$app/stores";
    import { user } from "$lib/stores";
    import IconAuthPlaceholder from "./IconAuthPlaceholder.svelte";
</script>

{#if !isLoggedIn()}
    <IconAuthPlaceholder link="/api/v1/auth/github/login?redirect_uri={$page.url}" />
{:else if !$user}
    <IconAuthPlaceholder />
{:else}
    <a class="btn-icon duration-300 hover:scale-110 hover:brightness-100" href="{base}/me">
        <img
            class="rounded-full"
            src="https://avatars.githubusercontent.com/u/{$user.github_id}"
            alt="avatar"
        />
    </a>
{/if}
