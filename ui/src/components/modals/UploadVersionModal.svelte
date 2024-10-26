<script lang="ts">
    import {
        getModalStore,
        ProgressRadial,
        InputChip,
        Autocomplete,
        popup,
        type PopupSettings,
    } from "@skeletonlabs/skeleton";
    import { addPackageAuthor, createVersion, getUser, searchUsers } from "$api";
    import type { ModLoader, PackageVersion, PackageVersionInit, User } from "$lib/types";
    import TablerIcon from "$components/icons/TablerIcon.svelte";
    import { currentPackage } from "$lib/stores";
    import { allLoaders, fixLoaderName } from "$lib/utils";
    import { onDestroy, onMount } from "svelte";
    import { elementPopup } from "$lib/ui/popups";
    import { goto } from "$app/navigation";

    const modals = getModalStore();
    let minecraftVersions = $state([]);

    let name = $state("");
    let version_number = $state("");
    let changelog = $state("");
    let kubejs: string[] = $state([]);
    let loaders: ModLoader[] = $state([]);
    let minecraft: string[] = $state([]);
    let files = $state<FileList>();

    let loading = $state(false);
    let errorMessages = $state<string[]>([]);

    let minecraftChips = $state("");
    let minecraftInput: InputChip = $state(null!);

    const minecraftAutocomplete: PopupSettings = {
        event: "focus-click",
        target: "minecraftAutocomplete",
        placement: "bottom",
    };

    const destroyHandlers: (() => void)[] = [];

    onMount(async () => {
        const manifest = await (
            await fetch("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json")
        ).json();

        minecraftVersions = manifest.versions.map((v: { id: string }) => v.id);

        const el = document.querySelector("[data-ref=minecraftInputChip]") as HTMLElement | null;

        if (el) destroyHandlers.push(elementPopup(el, minecraftAutocomplete));
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
            errorMessages.push("Version name is required");
        }

        if (version_number == "") {
            errorMessages.push("Version number is required");
        }

        if (loaders.length == 0) {
            errorMessages.push("At least one loader is required");
        }

        if (minecraft.length == 0) {
            errorMessages.push("At least one Minecraft version is required");
        }

        if (!file) {
            errorMessages.push("A file is required");
        }

        if (errorMessages.length > 0) {
            loading = false;
            return;
        }

        // TODO: KubeJS version input
        const data: PackageVersionInit = {
            name,
            version_number,
            changelog,
            kubejs,
            loaders,
            minecraft,
        };

        const fileData = await file!.arrayBuffer();
        let res: PackageVersion | undefined;
        
        try {
            res = await createVersion($currentPackage!.id, data, new Blob([fileData]));

            if (!res) {
                throw new Error("Failed to upload version");
            }
        } catch (err) {
            errorMessages.push("Failed to upload version");
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
    <div class="w-modal bg-secondary-700 relative rounded-lg p-8 shadow-xl">
        <header class="text-2xl font-bold">Upload Version</header>

        <form class="modal-form mt-4 rounded-md border p-4 border-transparent" onsubmit={submit} class:form-error={errorMessages.length > 0}>
            <input
                class="input variant-form-material mb-2 rounded-lg"
                type="text"
                placeholder="Version display name"
                disabled={loading}
                bind:value={name}
            />

            <input
                class="input variant-form-material rounded-lg"
                type="text"
                placeholder="Version number"
                disabled={loading}
                bind:value={version_number}
            />

            <div class="m-2 mt-4 flex flex-row items-center">
                <p class="mr-2 text-base">Supported Loaders</p>
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

            <InputChip
                bind:this={minecraftInput}
                bind:input={minecraftChips}
                bind:value={minecraft}
                name="chips"
                class="variant-form-material mt-4 !outline-none"
                placeholder="Minecraft versions"
                whitelist={minecraftVersions}
                data-ref="minecraftInputChip"
            />

            <div
                class="card variant-filled-secondary z-20 max-h-48 w-[calc(100%-4rem)] overflow-y-auto rounded-md p-2"
                tabindex="-1"
                data-popup="minecraftAutocomplete"
            >
                {#if minecraftVersions.length > 0}
                    <Autocomplete
                        bind:input={minecraftChips}
                        options={minecraftVersions.map((v) => ({ value: v, label: v }))}
                        denylist={minecraft}
                        on:selection={(ev) => minecraftInput.addChip(ev.detail.value)}
                    />
                {/if}
            </div>

            <textarea
                class="input variant-form-material mt-2 h-48 w-full rounded-lg"
                placeholder="Changelog"
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
                <p class="w-full">Error:</p>
                <ul class="ml-6 list-disc">
                    {#each errorMessages as message}
                        <li>{message}</li>
                    {/each}
                </ul>
            </div>
        {/if}

        <footer class="modal-footer mt-4 flex flex-row items-center">
            <button
                class="variant-soft-tertiary btn hover:variant-soft-primary mr-2 outline-none"
                disabled={loading}
                onclick={() => modals.close()}>Cancel</button
            >
            <button
                class="variant-primary btn hover:variant-filled-secondary flex flex-row items-center outline-none"
                disabled={loading}
                onclick={submit}
            >
                {#if loading}
                    <ProgressRadial width="w-4" class="mr-2" />
                {/if}
                Upload
            </button>
        </footer>
    </div>
{/if}

<style lang="postcss">
    :global(input) {
        outline: none !important;
    }
</style>
