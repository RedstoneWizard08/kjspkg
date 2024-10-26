<script lang="ts">
    import { _ } from "svelte-i18n";
    import { fly } from "svelte/transition";
    import {
        getModalStore,
        popup,
        type PopupSettings,
        ProgressRadial,
    } from "@skeletonlabs/skeleton";
    import { addPackageAuthor, getUser, searchUsers } from "$api";
    import type { User } from "$lib/types";
    import TablerIcon from "$components/icons/TablerIcon.svelte";
    import { currentPackage } from "$lib/stores";

    const modals = getModalStore();
    let username = $state("");
    let foundUsers: User[] = $state([]);
    let loading = $state(false);
    let errored = $state(false);
    let input: HTMLInputElement = $state(null!);

    const popupSettings: PopupSettings = {
        event: "focus-click",
        target: "popupFocusClick",
        placement: "bottom",
    };

    const errorPopupSettings: PopupSettings = {
        event: "hover",
        target: "errorPopup",
        placement: "bottom",
    };

    const refetchUsers = async () => {
        if (username == "") {
            foundUsers = [];
            return;
        }

        foundUsers = (await searchUsers(username)) ?? [];
    };

    const submit = async (ev: Event) => {
        ev.preventDefault();

        if (username == "" || loading || !$currentPackage) {
            return;
        }

        errored = false;
        loading = true;

        input.blur();

        const user = await getUser(username);

        if (!user) {
            errored = true;
            loading = false;
            return;
        }

        $currentPackage = await addPackageAuthor($currentPackage.id, user.id);
        loading = false;
        modals.close();
    };
</script>

{#if $modals[0]}
    <div class="w-modal-slim relative rounded-lg bg-secondary-700 p-8 shadow-xl">
        <header class="text-2xl font-bold">{$_("modal.add_author.title")}</header>

        <form class="modal-form mt-4" onsubmit={submit}>
            <div
                class="input-group input-group-divider grid-cols-[1fr_auto]"
                class:!border-error-500={errored}
            >
                <input
                    class="input rounded-lg"
                    type="text"
                    bind:value={username}
                    placeholder={$_("modal.add_author.placeholder.username")}
                    use:popup={popupSettings}
                    oninput={refetchUsers}
                    disabled={loading}
                    bind:this={input}
                />

                {#if errored}
                    <!-- svelte-ignore a11y_missing_attribute -->
                    <a class="[&>*]:pointer-events-none" use:popup={errorPopupSettings}>
                        <TablerIcon name="alert-triangle" />
                    </a>
                {/if}
            </div>

            <div class="card variant-filled-secondary p-4" data-popup="errorPopup">
                <p class="w-full">{$_("modal.add_author.error.user")}</p>
                <div class="variant-filled-secondary arrow"></div>
            </div>

            <div
                class="card variant-filled-surface w-[calc(100%-4rem)] p-4"
                data-popup="popupFocusClick"
                class:ml-7={errored}
                in:fly={{ y: 20 }}
            >
                {#if foundUsers.length > 0}
                    {#each foundUsers as user}
                        <!-- svelte-ignore a11y_click_events_have_key_events -->
                        <!-- svelte-ignore a11y_no_static_element_interactions -->
                        <div
                            class="card variant-filled-surface mb-2 flex cursor-pointer flex-row items-center p-2 transition-all hover:variant-soft-primary"
                            onclick={() => (username = user.username)}
                        >
                            <img
                                src="https://avatars.githubusercontent.com/u/{user.github_id}"
                                alt="author's profile afirst child cssvatar"
                                class="my-auto mr-4 aspect-square h-8 rounded-token"
                            />
                            {user.username}
                        </div>
                    {/each}
                {:else}
                    <div class="text-center">{$_("modal.add_author.error.no_users")}</div>
                {/if}

                <div class="variant-filled-surface arrow"></div>
            </div>
        </form>

        <footer class="modal-footer mt-4 flex flex-row items-center">
            <button
                class="variant-soft-tertiary btn mr-2 outline-none hover:variant-soft-primary"
                disabled={loading}
                onclick={() => modals.close()}>{$_("action.cancel")}</button
            >
            <button
                class="variant-primary btn flex flex-row items-center outline-none hover:variant-filled-secondary"
                disabled={loading}
                onclick={submit}
            >
                {#if loading}
                    <ProgressRadial width="w-4" class="mr-2" />
                {/if}
                {$_("action.add")}
            </button>
        </footer>
    </div>
{/if}
