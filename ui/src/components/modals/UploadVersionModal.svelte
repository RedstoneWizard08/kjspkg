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
    import type { ModLoader, PackageVersion, PackageVersionInit } from "$lib/types";
    import { currentPackage } from "$lib/stores";
    import { allLoaders, fixLoaderName } from "$lib/utils";
    import { onDestroy, onMount } from "svelte";
    import { elementPopup, type PopupControls } from "$lib/ui/popups";
    import { goto } from "$app/navigation";
    import TablerIcon from "$components/icons/TablerIcon.svelte";

    const modals = getModalStore();
    let minecraftVersions = $state<[string, boolean][]>([]);

    let name = $state("");
    let version_number = $state("");
    let changelog = $state("");
    let kubejs: string[] = $state([]);
    let loaders: ModLoader[] = $state([]);
    let minecraft: string[] = $state([]);
    let files = $state<FileList>();

    let loading = $state(false);
    let snapshots = $state(false);
    let minecraftOpen = $state(false);
    let errorMessages = $state<string[]>([]);

    let minecraftChips = $state("");
    let minecraftInput: InputChip = $state(null!);
    let popup: PopupControls | undefined = $state(undefined);

    const minecraftAutocomplete: PopupSettings = {
        event: "focus-click",
        target: "minecraftAutocomplete",
        placement: "bottom",
    };

    const destroyHandlers: (() => void)[] = [];
    const releaseVersions = $derived(minecraftVersions.filter((v) => v[1]).map((v) => v[0]));
    const snapshotVersions = $derived(minecraftVersions.map((v) => v[0]));
    const shownMinecraftVersions = $derived(snapshots ? snapshotVersions : releaseVersions);

    onMount(async () => {
        const manifest = await (
            await fetch("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json")
        ).json();

        minecraftVersions = manifest.versions.map((v: { id: string; type: string }) => [
            v.id,
            v.type == "release",
        ]);

        const el = document.querySelector("[data-ref=minecraftInputChip]") as HTMLElement | null;

        el?.addEventListener("focus", () => (minecraftOpen = true));
        el?.addEventListener("blur", () => (minecraftOpen = false));

        if (el) {
            popup = elementPopup(el, minecraftAutocomplete);

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

        if (minecraft.length == 0) {
            errorMessages.push($_("modal.upload_version.error.minecraft"));
        }

        if (!file) {
            errorMessages.push($_("modal.upload_version.error.file"));
        }

        if (errorMessages.length > 0) {
            loading = false;
            return;
        }

        // TODO: KubeJS version input
        const data: PackageVersionInit = {
            name,
            version_number,
            changelog: changelog == "" ? undefined : changelog,
            kubejs,
            loaders,
            minecraft,
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

        goto(`/p/${$currentPackage!.id}/v/${res.id}`);

        loading = false;
        modals.close();
    };

    const toggleLoader = (loader: ModLoader) => {
        if (loaders.includes(loader)) {
            loaders = loaders.filter((l) => l !== loader);
        } else {
            loaders.push(loader);
        }
    };
</script>

{#if $modals[0]}
    <div
        class="w-modal relative max-h-[96vh] !overflow-scroll rounded-lg bg-secondary-700 p-8 shadow-xl"
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
                {#each allLoaders as loader}
                    <button
                        type="button"
                        class="chip mx-2 text-base !outline-none {loaders.includes(loader)
                            ? 'variant-filled-primary'
                            : 'variant-soft'}"
                        onclick={() => toggleLoader(loader)}>{fixLoaderName(loader)}</button
                    >
                {/each}
            </div>

            <div
                class="mt-4 grid w-full grid-cols-[1fr_auto] overflow-hidden transition duration-200"
            >
                <InputChip
                    bind:this={minecraftInput}
                    bind:input={minecraftChips}
                    bind:value={minecraft}
                    name="chips"
                    class="variant-form-material !min-w-fit !outline-none"
                    placeholder={$_("modal.upload_version.placeholder.minecraft")}
                    whitelist={snapshotVersions}
                    data-ref="minecraftInputChip"
                />

                <button
                    type="button"
                    class="variant-form-material flex items-center justify-between border-surface-500 px-4 !outline-none"
                    onclick={() =>
                        (
                            document.querySelector(
                                "[data-ref=minecraftInputChip]",
                            ) as HTMLElement | null
                        )?.focus()}
                >
                    <TablerIcon name="caret-down" rotated={minecraftOpen} />
                </button>
            </div>

            <div class="my-2 flex flex-row items-center justify-between">
                <div class="flex flex-row items-center justify-start">
                    <input class="checkbox" type="checkbox" bind:checked={snapshots} />
                    <p class="ml-2">Show Snapshots</p>
                </div>

                <div class="flex flex-row items-center justify-end">
                    <button
                        type="button"
                        class="variant-filled-primary btn btn-sm !outline-none"
                        onclick={() => (minecraft = shownMinecraftVersions)}
                    >
                        Select All
                    </button>

                    <button
                        type="button"
                        class="variant-filled-error btn btn-sm ml-2 !outline-none"
                        onclick={() => (minecraft = [])}
                    >
                        Clear
                    </button>
                </div>
            </div>

            <div
                class="card variant-filled-secondary z-20 ml-7 max-h-48 w-[calc(100%-4rem)] overflow-y-auto rounded-md p-2"
                tabindex="-1"
                data-popup="minecraftAutocomplete"
            >
                {#if minecraftVersions.length > 0}
                    <Autocomplete
                        bind:input={minecraftChips}
                        options={shownMinecraftVersions.map((v) => ({ value: v, label: v }))}
                        denylist={minecraft}
                        on:selection={(ev) => minecraftInput.addChip(ev.detail.value)}
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
                accept=".kjspkg,.tgz,.tar.gz"
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
