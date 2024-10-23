<script lang="ts">
    import { currentScrollPosition } from "$lib/stores";
    import { AppShell, Modal, Toast } from "@skeletonlabs/skeleton";
    import { browser } from "$app/environment";
    import { locale, waitLocale } from "svelte-i18n";
    import type { LayoutLoad } from "./$types";

    const { children } = $props();

    export const load: LayoutLoad = async () => {
        if (browser) {
            locale.set(window.navigator.language);
        }
        
        await waitLocale();
    };
</script>

<Toast position="br" max={8} />
<Modal />

<AppShell
    slotSidebarLeft="hidden xl:block"
    slotPageFooter="p-2 flex justify-between"
    onscroll={(e: WheelEvent) =>
        ($currentScrollPosition = {
            x: (e.currentTarget as Element).scrollLeft,
            y: (e.currentTarget as Element).scrollTop,
        })}
>
    {@render children?.()}

    <svelte:fragment slot="pageFooter">
        <span class="hidden md:inline">
            <a
                href="https://github.com/Modern-Modpacks/kjspkg"
                class="anchor no-underline"
                target="_blank">KJSPKG @ GitHub</a
            >
            &bull;
            <a
                href="https://github.com/Modern-Modpacks/kjspkg-lookup"
                class="anchor no-underline"
                target="_blank">KJSPKG Lookup @ GitHub</a
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
            > &lt;3
        </span>
    </svelte:fragment>
</AppShell>
