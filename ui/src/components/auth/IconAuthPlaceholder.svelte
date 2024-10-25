<script lang="ts">
    import { IconUser } from "@tabler/icons-svelte";

    const { link }: { link?: string } = $props();
</script>

{#if link != null}
    <a href={link} class="variant-soft-primary btn-icon">
        <IconUser />
    </a>
{:else}
    <div class="variant-soft-primary btn-icon cursor-pointer"><IconUser /></div>
{/if}
