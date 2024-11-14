<script lang="ts">
    import { _ } from "svelte-i18n";
    import { Drawer, getDrawerStore } from "@skeletonlabs/skeleton";
    import TablerIcon from "$components/icons/TablerIcon.svelte";

    const drawerStore = getDrawerStore();

    const closeDrawer = () => {
        drawerStore.close();
    };
</script>

<Drawer>
    {#if $drawerStore.id == "home"}
        <div class="flex flex-col p-4">
            <p class="flex items-center gap-2">
                <img src="/kjspkg.png" alt="logo" class="aspect-square w-8 min-w-8 rounded-token" />
                <span>KJSPKG</span>
                <span class="variant-filled-secondary badge">{$_("site.beta")}</span>
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

                <a
                    href="/install"
                    class="btn-primary transition-duration-300 variant-soft-primary btn mr-4 flex flex-row items-center justify-start outline-none transition-all hover:variant-filled-secondary"
                    onclick={closeDrawer}
                >
                    <TablerIcon name="download" />
                    <span>{$_("site.install")}</span>
                </a>
            </div>
        </div>
    {/if}
</Drawer>
