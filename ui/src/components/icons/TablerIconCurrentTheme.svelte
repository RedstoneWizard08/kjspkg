<script lang="ts">
    import { userPreferencesStore } from "$lib/stores";
    import TablerIconMoon from "./TablerIconMoon.svelte";
    import TablerIconSun from "./TablerIconSun.svelte";
</script>

{#if $userPreferencesStore.lightMode}
    <TablerIconSun />
{:else}
    <TablerIconMoon />
{/if}
