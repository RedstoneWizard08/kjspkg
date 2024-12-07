<script lang="ts">
    import { getPackages } from "$api";
    import type { PackageData } from "$lib/types";
    import { capText, splitToRows } from "$lib/utils";
    import { onMount } from "svelte";

    const rows = 3;
    const maxPkgs = 30;
    const rowElements: HTMLDivElement[] = [];

    let projects: PackageData[] = $state([]);
    let selected = $derived(splitToRows(projects, rows));

    onMount(async () => {
        const pkgs = await getPackages();

        if (pkgs) {
            projects = pkgs.length >= maxPkgs ? pkgs.slice(0, maxPkgs) : pkgs;
        }
    });

    const inHandler = (idx: number) => {
        return () => {
            const el = rowElements[idx];

            if (el) {
                el.style.animationPlayState = "paused";
            }
        };
    };

    const outHandler = (idx: number) => {
        return () => {
            const el = rowElements[idx];

            if (el) {
                el.style.animationPlayState = "running";
            }
        };
    };
</script>

<div class="mt-16 flex flex-col space-y-4">
    {#each selected as items, index}
        <div
            class="hide-scrollbar flex w-screen select-none flex-row gap-6 overflow-hidden whitespace-nowrap"
        >
            <div
                class="flex min-w-full flex-shrink-0 animate-scroll gap-6 whitespace-nowrap"
                class:anim-reverse={!(index % 2 == 0)}
                class:anim-mid={!(index % 2 == 0)}
                bind:this={rowElements[index]}
            >
                {#each items as pkg}
                    <!-- svelte-ignore a11y_mouse_events_have_key_events -->
                    <a
                        class="flex cursor-pointer flex-row gap-4 rounded-xl border-[1px] border-surface-500 bg-surface-700 p-4 transition-all hover:bg-surface-500"
                        href="/p/{pkg.slug}"
                        onmouseover={inHandler(index)}
                        onmouseleave={outHandler(index)}
                    >
                        <!-- <Avatar :src="project.icon_url" :alt="project.title" size="sm" loading="lazy" /> -->
                        {#if pkg.authors[0].github_id == -1}
                            <img
                                src="/modhost.png"
                                alt="author's profile avatar"
                                class="my-auto mr-1 aspect-square h-10 rounded-token"
                            />
                        {:else}
                            <img
                                src={`https://avatars.githubusercontent.com/u/${pkg.authors[0].github_id}`}
                                alt="author's profile avatar"
                                class="my-auto mr-1 aspect-square h-10 rounded-token"
                            />
                        {/if}
                        <div class="project-info flex flex-col">
                            <span class="title font-bold">
                                {pkg.name}
                            </span>
                            <span class="description">
                                {capText(pkg.description, 40)}
                            </span>
                        </div>
                    </a>
                {/each}
            </div>
        </div>
    {/each}
</div>
