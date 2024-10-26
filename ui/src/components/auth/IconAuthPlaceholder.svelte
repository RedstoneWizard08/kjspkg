<script lang="ts">
    import TablerIcon from "$components/icons/TablerIcon.svelte";

    const { link }: { link?: string } = $props();
</script>

{#if link != null}
    <a href={link} class="variant-soft-primary btn-icon">
        <TablerIcon name="user" />
    </a>
{:else}
    <div class="variant-soft-primary btn-icon cursor-pointer"><TablerIcon name="user" /></div>
{/if}
