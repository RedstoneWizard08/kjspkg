<script lang="ts">
    import Icon from "@iconify/svelte";

    const { link }: { link?: string } = $props();
</script>

{#if link != null}
    <a href={link} class="variant-soft-primary btn-icon">
        <Icon icon="tabler:user" height="24" />
    </a>
{:else}
    <div class="variant-soft-primary btn-icon cursor-pointer">
        <Icon icon="tabler:user" height="24" />
    </div>
{/if}
