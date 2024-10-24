<script lang="ts">
    import { _ } from "svelte-i18n";
    import { base } from "$app/paths";
    import { page } from "$app/stores";
    import { filteredStore } from "$lib/stores";
    import { flip } from "svelte/animate";
    import { fade, slide } from "svelte/transition";

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
        class="card hover:variant-soft-primary flex p-4"
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
                class="rounded-token my-auto mr-4 aspect-square h-8"
                in:slide={{ axis: "x" }}
            />
        {/if}
        <dl class="my-auto">
            <dt class="mb-1 select-text font-bold">{pkg.name}</dt>
            <dd class="text-sm opacity-50">
                <!-- {#if branch && showDetails}
                    {$langKeyStore["list.detailed.on_branch"]}
                    <span class="select-text">{branch.substring(1)}</span>
                {/if}
                {#if path && showDetails}
                    {$langKeyStore["list.detailed.at_path"]}
                    <span class="select-text">{path.substring(1)}</span>
                {/if}
                {#if name != repo && showDetails}
                    {$langKeyStore["list.detailed.in_repo"]} <span class="select-text">{repo}</span>
                {/if} -->

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
                    {pkg.views == 1
                        ? $_("list.view_singular")
                        : $_("list.view_plural")}</span
                >
            </dd>
        </dl>
    </a>
{/each}
