<script lang="ts">
    import { _ } from "svelte-i18n";
    import {
        getModalStore,
        ProgressRadial,
        InputChip,
        Autocomplete,
        type PopupSettings,
    } from "@skeletonlabs/skeleton";
    import { createVersion } from "$api";
    import type { PackageVersion, PackageVersionInit } from "$lib/types";
    import { currentPackage } from "$lib/stores";
    import { onDestroy, onMount } from "svelte";
    import { elementPopup, type PopupControls } from "$lib/ui/popups";
    import { goto } from "$app/navigation";
    import { siteConfig } from "$lib/config";
    import { gameVersions as allGameVersions } from "$lib/versions";
    import { modLoaders } from "$lib/loaders";
    import Icon from "@iconify/svelte";

    const modals = getModalStore();

    let name = $state("");
    let version_number = $state("");
    let changelog = $state("");
    let loaders: string[] = $state([]);
    let versions: string[] = $state([]);
    let files = $state<FileList>();

    let loading = $state(false);
    let snapshots = $state(false);
    let versionsOpen = $state(false);
    let errorMessages = $state<string[]>([]);

    let versionChips = $state("");
    let versionsInput: InputChip = $state(null!);
    let popup: PopupControls | undefined = $state(undefined);

    const versionsAutocomplete: PopupSettings = {
        event: "focus-click",
        target: "versionsAutocomplete",
        placement: "bottom",
    };

    const gameVersions: [string, boolean][] = $derived(
        ($allGameVersions || []).map((v: { id: string; beta: boolean }) => [v.id, !v.beta]),
    );

    const destroyHandlers: (() => void)[] = [];
    const releaseVersions = $derived(gameVersions.filter((v) => v[1]).map((v) => v[0]));
    const betaVersions = $derived(gameVersions.map((v) => v[0]));
    const shownGameVersions = $derived(snapshots ? betaVersions : releaseVersions);

    onMount(async () => {
        const el = document.querySelector("[data-ref=versionInputChip]") as HTMLElement | null;

        el?.addEventListener("focus", () => (versionsOpen = true));
        el?.addEventListener("blur", () => (versionsOpen = false));

        if (el) {
            popup = elementPopup(el, versionsAutocomplete);

            destroyHandlers.push(popup.destroy);
        }
    });

    onDestroy(() => {
        for (const handler of destroyHandlers) handler();
    });

    const submit = async (ev: Event) => {
        ev.preventDefault();
        loading = true;
        errorMessages = [];

        const file = files?.[0];

        if (name == "") {
            errorMessages.push($_("modal.upload_version.error.name"));
        }

        if (version_number == "") {
            errorMessages.push($_("modal.upload_version.error.version_number"));
        }

        if (loaders.length == 0) {
            errorMessages.push($_("modal.upload_version.error.loaders"));
        }

        if (versions.length == 0) {
            errorMessages.push($_("modal.upload_version.error.game"));
        }

        if (!file) {
            errorMessages.push($_("modal.upload_version.error.file"));
        }

        if (errorMessages.length > 0) {
            loading = false;
            return;
        }

        const data: PackageVersionInit = {
            name,
            version_number,
            changelog: changelog == "" ? undefined : changelog,
            loaders,
            game_versions: versions,
        };

        const fileData = await file!.arrayBuffer();
        let res: PackageVersion | undefined;

        try {
            res = await createVersion($currentPackage!.id, data, new Blob([fileData]));

            if (!res) {
                throw new Error($_("modal.upload_version.error.api"));
            }
        } catch (err) {
            errorMessages.push($_("modal.upload_version.error.api"));
            loading = false;
            return;
        }

        goto(`/p/${$currentPackage!.id}/versions/${res.id}`);

        loading = false;
        modals.close();
    };

    const toggleLoader = (loader: string) => {
        if (loaders.includes(loader)) {
            loaders = loaders.filter((l) => l !== loader);
        } else {
            loaders.push(loader);
        }
    };
</script>

{#if $modals[0]}
    <div
        class="w-modal relative max-h-[96vh] !overflow-scroll rounded-lg bg-surface-700 p-8 shadow-xl"
    >
        <header class="text-2xl font-bold">{$_("modal.upload_version.title")}</header>

        <form
            class="modal-form mt-4 h-full rounded-md border border-transparent p-4"
            onsubmit={submit}
            class:form-error={errorMessages.length > 0}
        >
            <input
                class="input variant-form-material mb-2 rounded-lg"
                type="text"
                placeholder={$_("modal.upload_version.placeholder.name")}
                disabled={loading}
                bind:value={name}
            />

            <input
                class="input variant-form-material rounded-lg"
                type="text"
                placeholder={$_("modal.upload_version.placeholder.version_number")}
                disabled={loading}
                bind:value={version_number}
            />

            <p class="my-2 lg:hidden">{$_("modal.upload_version.placeholder.loaders")}</p>

            <div class="flex flex-row items-center lg:m-2 lg:mt-4">
                <p class="mr-2 hidden lg:block">{$_("modal.upload_version.placeholder.loaders")}</p>
                {#each $modLoaders ?? [] as loader}
                    <button
                        type="button"
                        class="chip mx-1 text-base !outline-none {loaders.includes(loader.id)
                            ? 'variant-filled-primary'
                            : 'variant-soft'}"
                        onclick={() => toggleLoader(loader.id)}>{loader.name}</button
                    >
                {/each}
            </div>

            <div
                class="mt-4 grid w-full grid-cols-[1fr_auto] overflow-hidden transition duration-200"
            >
                <InputChip
                    bind:this={versionsInput}
                    bind:input={versionChips}
                    bind:value={versions}
                    name="chips"
                    class="variant-form-material !min-w-fit !outline-none"
                    placeholder={$_("modal.upload_version.placeholder.game")}
                    whitelist={betaVersions}
                    data-ref="versionInputChip"
                />

                <button
                    type="button"
                    class="variant-form-material flex items-center justify-between border-surface-500 px-4 !outline-none"
                    onclick={() =>
                        (
                            document.querySelector(
                                "[data-ref=versionInputChip]",
                            ) as HTMLElement | null
                        )?.focus()}
                >
                    <Icon icon="tabler:caret-down" height="24" rotate={versionsOpen ? 180 : 0} />
                </button>
            </div>

            <div class="my-2 flex flex-row items-center justify-between">
                <div class="flex flex-row items-center justify-start">
                    <input class="checkbox" type="checkbox" bind:checked={snapshots} />
                    <p class="ml-2">{$_(`modal.upload_version.checkbox.${siteConfig.betaName}`)}</p>
                </div>

                <div class="flex flex-row items-center justify-end">
                    <button
                        type="button"
                        class="variant-filled-primary btn btn-sm !outline-none"
                        onclick={() => (versions = shownGameVersions)}
                    >
                        {$_("modal.upload_version.select_all")}
                    </button>

                    <button
                        type="button"
                        class="variant-filled-error btn btn-sm ml-2 !outline-none"
                        onclick={() => (versions = [])}
                    >
                        {$_("modal.upload_version.clear")}
                    </button>
                </div>
            </div>

            <div
                class="card variant-filled-secondary z-20 ml-7 max-h-48 w-[calc(100%-4rem)] overflow-y-auto rounded-md p-2"
                tabindex="-1"
                data-popup="versionsAutocomplete"
            >
                {#if gameVersions.length > 0}
                    <Autocomplete
                        bind:input={versionChips}
                        options={shownGameVersions.map((v) => ({ value: v, label: v }))}
                        denylist={versions}
                        on:selection={(ev) => versionsInput.addChip(ev.detail.value)}
                    />
                {/if}
            </div>

            <textarea
                class="input variant-form-material mt-2 h-48 w-full rounded-lg"
                placeholder={$_("modal.upload_version.placeholder.changelog")}
                disabled={loading}
                bind:value={changelog}
            ></textarea>

            <input
                class="input variant-form-material mt-1 rounded-lg"
                type="file"
                accept=".tgz,.tar.gz"
                multiple
                disabled={loading}
                bind:files
            />
        </form>

        {#if errorMessages.length > 0}
            <div class="card variant-filled-error mt-4 p-4">
                <p class="w-full">{$_("errors.prefix")}</p>
                <ul class="ml-6 list-disc">
                    {#each errorMessages as message}
                        <li>{message}</li>
                    {/each}
                </ul>
            </div>
        {/if}

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
                {$_("action.upload")}
            </button>
        </footer>
    </div>
{/if}

<style lang="postcss">
    :global(input) {
        outline: none !important;
    }
</style>
