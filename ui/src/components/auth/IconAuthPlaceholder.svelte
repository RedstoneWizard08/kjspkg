<script lang="ts">
    import { IconUser } from "@tabler/icons-svelte";

    const { link }: { link?: string } = $props();
</script>

{#if link != null}
    <a href={link} class="btn-icon variant-soft-primary">
        <IconUser />
    </a>
{:else}
    <div class="btn-icon variant-soft-primary cursor-pointer"><IconUser /></div>
{/if}
