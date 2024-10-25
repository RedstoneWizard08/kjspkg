<script lang="ts">
    import "../app.pcss";
    import "carta-md/default.css";
    import { currentScrollPosition, updateUserStore, userPreferencesStore } from "$lib/stores";
    import {
        AppShell,
        Modal,
        Toast,
        storePopup,
        initializeStores,
        ProgressRadial,
    } from "@skeletonlabs/skeleton";
    import { computePosition, autoUpdate, offset, shift, flip, arrow } from "@floating-ui/dom";
    import { afterNavigate, beforeNavigate, onNavigate } from "$app/navigation";
    import { onMount, type Snippet } from "svelte";
    import { fly } from "svelte/transition";
    import HeaderBar from "$components/ui/HeaderBar.svelte";
    import ContextMenu from "$components/ui/ContextMenu.svelte";
    import Sidebar from "$components/ui/Sidebar.svelte";

    const { data, children }: { data: any; children: Snippet } = $props();

    storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });
    initializeStores();

    let navigating = $state(false);

    onMount(async () => {
        if ($userPreferencesStore.lightMode) document.documentElement.classList.remove("dark");

        document.body.dataset.theme = $userPreferencesStore.theme ?? "kjspkg";

        await updateUserStore();
    });

    beforeNavigate(() => (navigating = true));
    afterNavigate(() => (navigating = false));

    onNavigate((navigation) => {
        if (navigation.to?.route.id == navigation.from?.route.id) return;
        if (!document.startViewTransition) return;

        return new Promise((resolve) => {
            document.startViewTransition(async () => {
                resolve();
                await navigation.complete;
            });
        });
    });
</script>

<Toast position="br" max={8} />
<ContextMenu />
<Modal />

{#if navigating}
    <div
        class="card variant-soft-success absolute bottom-8 right-8 z-40 flex w-16 p-4"
        in:fly={{ x: 40, delay: 200 }}
        out:fly={{ x: 40 }}
    >
        <ProgressRadial stroke={40} meter="stroke-success-500" track="stroke-success-500/30" />
    </div>
{/if}

<AppShell
    slotSidebarLeft="hidden xl:block"
    slotPageFooter="p-2 flex justify-between"
    onscroll={(e: WheelEvent) =>
        ($currentScrollPosition = {
            x: (e.currentTarget as Element).scrollLeft,
            y: (e.currentTarget as Element).scrollTop,
        })}
>
    <svelte:fragment slot="header">
        <HeaderBar />
    </svelte:fragment>

    <svelte:fragment slot="sidebarLeft">
        <Sidebar />
    </svelte:fragment>

    <div
        class="container relative mx-auto flex min-h-full max-w-screen-lg flex-col space-y-2 p-4 md:p-10"
    >
        {#key data.href}
            {@render children?.()}
        {/key}
    </div>

    <svelte:fragment slot="pageFooter">
        <span class="hidden md:inline">
            <a
                href="https://github.com/Modern-Modpacks/kjspkg"
                class="anchor no-underline"
                target="_blank">KJSPKG @ GitHub</a
            >
            &bull;
            <a href="https://modernmodpacks.site" class="anchor no-underline" target="_blank"
                >Modern Modpacks</a
            >
        </span>

        <span class="mt-auto hidden text-sm opacity-50 md:inline">
            Website designed with love by <a
                href="https://github.com/tizu69"
                class="anchor no-underline"
                target="_blank">tizu69</a
            >
            and
            <a
                href="https://github.com/RedstoneWizard08"
                class="anchor no-underline"
                target="_blank">RedstoneWizard08</a
            > &lt;3
        </span>
    </svelte:fragment>
</AppShell>
