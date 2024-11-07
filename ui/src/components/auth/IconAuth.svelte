<script lang="ts">
    import { _, locales, locale } from "svelte-i18n";
    import { page } from "$app/stores";
    import TablerIconUpload from "$components/icons/TablerIconUpload.svelte";
    import {
        contextMenu,
        openContextMenu,
        type ContextMenuItem,
        type ContextMenuProps,
    } from "$lib/contextMenu";
    import { user, userPreferencesStore } from "$lib/stores";
    import { getModalStore } from "@skeletonlabs/skeleton";
    import TablerIcon from "$components/icons/TablerIcon.svelte";
    import TablerIconWorld from "$components/icons/TablerIconWorld.svelte";
    import TablerIconCheck from "$components/icons/TablerIconCheck.svelte";
    import IconBlank from "$components/icons/IconBlank.svelte";
    import TablerIconBack from "$components/icons/TablerIconArrowLeft.svelte";
    import TablerIconColorSwatch from "$components/icons/TablerIconColorSwatch.svelte";
    import { goto } from "$app/navigation";
    import TablerIconLogin2 from "$components/icons/TablerIconLogin2.svelte";
    import TablerIconLogin from "$components/icons/TablerIconLogin.svelte";
    import { setToken } from "$api";
    import TablerIconUser from "$components/icons/TablerIconUser.svelte";
    import TablerIconCurrentTheme from "$components/icons/TablerIconCurrentTheme.svelte";

    const modals = getModalStore();

    const openCreateModal = () => {
        modals.trigger({
            type: "component",
            component: "createPackage",
        });
    };

    const langContextMenu = $derived({
        initiator: "left" as const,
        items: [
            {
                type: "ITEM",
                label: "Back",
                icon: TablerIconBack,
                action: () => openContextMenu(userContextMenu),
            },

            { type: "SEPARATOR", header: $_("menu.language") },

            ...$locales.map((lang) => ({
                type: "ITEM",
                label: $_("name", { locale: lang }),
                icon: $userPreferencesStore.locale == lang ? TablerIconCheck : IconBlank,
                action: () => {
                    $userPreferencesStore.locale = lang;
                    $locale = lang;
                },
            })),
        ] as ContextMenuItem[],
    });

    const themeContextMenu = $derived({
        initiator: "left" as const,
        items: [
            {
                type: "ITEM",
                label: "Back",
                icon: TablerIconBack,
                action: () => openContextMenu(userContextMenu),
            },

            { type: "SEPARATOR", header: $_("menu.theme") },

            ...[
                { label: "KJSPKG", name: "kjspkg" },
                { label: "KJSPKG Lighter", name: "kjspkg-lighter" },
                { label: "G_cat", name: "kjspkg-gcat" },
                {},
                { label: "Wintry", name: "wintry" },
                { label: "Crimson", name: "crimson" },
                { label: "Serenity", name: "serenity" },
                { label: "Hamlindigo", name: "hamlindigo" },
                { label: "Modern", name: "modern" },
                { label: "Rocket", name: "rocket" },
                { label: "Sahara", name: "sahara" },
                { label: "Seafoam", name: "seafoam" },
                { label: "Skeleton", name: "skeleton" },
                { label: "Vintage", name: "vintage" },
            ].map(({ label, name }) => {
                if (!label || !name) return { type: "SEPARATOR" };

                return {
                    type: "ITEM" as const,
                    label,
                    icon: $userPreferencesStore.theme == name ? TablerIconCheck : IconBlank,
                    action: () => {
                        document.documentElement.classList.add("color-animated");
                        $userPreferencesStore.theme = name;
                        document.body.dataset.theme = $userPreferencesStore.theme ?? "kjspkg";
                    },
                };
            }),
        ] as ContextMenuItem[],
    });

    const userContextMenu: ContextMenuProps = $derived({
        initiator: "left",
        items: (() => {
            const out: ContextMenuItem[] = [];

            if ($user) {
                out.push(
                    {
                        type: "ITEM",
                        label: "Your Profile",
                        icon: TablerIconUser,
                        action: () => goto(`/u/${$user.username}`),
                    },
                    {
                        type: "ITEM",
                        label: "Create Package",
                        icon: TablerIconUpload,
                        action: openCreateModal,
                    },
                );
            } else {
                out.push({
                    type: "ITEM",
                    label: "Sign In",
                    icon: TablerIconLogin2,
                    action: () => goto(`/api/v1/auth/github/login?redirect_uri=${$page.url}`),
                });
            }

            out.push(
                {
                    type: "SEPARATOR",
                },
                {
                    type: "ITEM",
                    label: "Change Language",
                    icon: TablerIconWorld,
                    action: () => openContextMenu(langContextMenu),
                },
                {
                    type: "ITEM",
                    label: "Change Theme",
                    icon: TablerIconColorSwatch,
                    action: () => openContextMenu(themeContextMenu),
                },
                // { // This is REALLY buggy right now with the colors, I'm not even sure if it's worth it.
                //     type: "ITEM",
                //     label: $userPreferencesStore.lightMode ? "Light Mode" : "Dark Mode",
                //     icon: TablerIconCurrentTheme,
                //     action: () => $userPreferencesStore.lightMode = !$userPreferencesStore.lightMode,
                // },
            );

            if ($user) {
                out.push(
                    {
                        type: "SEPARATOR",
                    },
                    {
                        type: "ITEM",
                        label: "Sign Out",
                        icon: TablerIconLogin,
                        action: () => {
                            document.cookie = "";
                            setToken(undefined);
                            window.location.reload();
                        },
                    },
                );
            }

            return out;
        })(),
    });
</script>

{#if !$user}
    <!-- href={"/api/v1/auth/github/login?redirect_uri={$page.url}"} -->
    <button class="variant-soft-primary btn-icon" use:contextMenu={userContextMenu}>
        <TablerIcon name="user" />
    </button>
{:else}
    <!-- href="{base}/me" -->
    <button
        class="btn-icon duration-300 hover:scale-110 hover:brightness-100"
        use:contextMenu={userContextMenu}
    >
        <img
            class="rounded-full"
            src="https://avatars.githubusercontent.com/u/{$user.github_id}"
            alt="avatar"
        />
    </button>
{/if}
