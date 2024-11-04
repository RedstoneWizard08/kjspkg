<script lang="ts">
    import { _ } from "svelte-i18n";
    import { base } from "$app/paths";
    import { page } from "$app/stores";
    import { filteredStore } from "$lib/stores";
    import { flip } from "svelte/animate";
    import { fade, slide } from "svelte/transition";
    import { formatDate } from "$lib/utils";

    interface Props {
        startFrom?: number;
        maxCount?: number;
        customHeight?: number;
        compact?: boolean;
        showAvatar?: boolean;
        showDetails?: boolean;
        showName?: boolean;
        select?: (id: number) => void | Promise<void>;
    }

    const {
        startFrom = 0,
        maxCount = Infinity,
        customHeight,
        compact = false,
        showName = true,
        showDetails = true,
        showAvatar = true,
        select,
    }: Props = $props();
</script>

{#each [...$filteredStore].slice(startFrom, maxCount) as pkg, i (pkg)}
    <a
        href={`${base}/p/${pkg.slug}`}
        class="card flex p-4 hover:variant-soft-primary"
        class:flex-col={compact}
        onclick={() => select?.(pkg.id)}
        class:!variant-filled-primary={$page.url.searchParams.get("id") == pkg.name}
        animate:flip={{ duration: 500 }}
        transition:fade={{ duration: 300 }}
        style={customHeight != null ? `height: ${customHeight}rem` : ""}
    >
        {#if showAvatar && !compact}
            <img
                src={`https://avatars.githubusercontent.com/u/${pkg.authors[0].github_id}`}
                alt="author's profile avatar"
                class="my-auto mr-4 aspect-square h-8 rounded-token"
                in:slide={{ axis: "x" }}
            />
        {/if}
        <dl class="my-auto">
            <dt class="mb-1 select-text font-bold">{pkg.name}</dt>
            <dd class="text-sm opacity-50">
                {#if showName}
                    {$_("list.by")} <span class="select-text">{pkg.authors[0].username}</span>
                {/if}
            </dd>
            <dd class="text-sm opacity-50">
                <span
                    >{pkg.downloads}
                    {pkg.downloads == 1
                        ? $_("list.download_singluar")
                        : $_("list.download_plural")}</span
                >
                &bull;
                <span
                    >{pkg.views}
                    {pkg.views == 1 ? $_("list.view_singular") : $_("list.view_plural")}</span
                >
            </dd>
            {#if showDetails}
                <dd class="text-sm opacity-50">
                    {$_("list.published")}
                    <span class="select-text">{formatDate(new Date(pkg.created_at))}</span>
                </dd>
                <dd class="text-sm opacity-50">
                    {$_("list.updated")}
                    <span class="select-text">{formatDate(new Date(pkg.updated_at))}</span>
                </dd>
            {/if}
        </dl>
    </a>
{/each}
