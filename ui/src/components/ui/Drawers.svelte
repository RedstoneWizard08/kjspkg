<script lang="ts">
    import { _ } from "svelte-i18n";
    import { Drawer, getDrawerStore } from "@skeletonlabs/skeleton";
    import TablerIcon from "$components/icons/TablerIcon.svelte";
    import { siteConfig } from "$lib/config";

    const drawerStore = getDrawerStore();

    const closeDrawer = () => {
        drawerStore.close();
    };
</script>

<Drawer>
    {#if $drawerStore.id == "home"}
        <div class="flex flex-col p-4">
            <p class="flex items-center gap-2">
                <img
                    src="/favicon.png"
                    alt="logo"
                    class="aspect-square w-8 min-w-8 rounded-token"
                />
                <span>{siteConfig.siteName}</span>

                {#if siteConfig.showBeta}
                    <span class="variant-filled-secondary badge">{$_("site.beta")}</span>
                {/if}
            </p>

            <div class="mt-4 flex flex-col space-y-4">
                <a
                    href="/"
                    class="btn-primary transition-duration-300 variant-soft-primary btn mr-4 flex flex-row items-center justify-start outline-none transition-all hover:variant-filled-secondary"
                    onclick={closeDrawer}
                >
                    <TablerIcon name="home" />
                    <span>{$_("search.home")}</span>
                </a>

                <a
                    href="/s"
                    class="btn-primary transition-duration-300 variant-soft-primary btn mr-4 flex flex-row items-center justify-start outline-none transition-all hover:variant-filled-secondary"
                    onclick={closeDrawer}
                >
                    <TablerIcon name="search" />
                    <span>{$_("search.browse")}</span>
                </a>
            </div>
        </div>
    {/if}
</Drawer>
