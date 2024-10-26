<script lang="ts">
    import { getModalStore, ProgressRadial } from "@skeletonlabs/skeleton";
    import { createPackage, getPackage } from "$api";
    import type { NewPackage, PackageData } from "$lib/types";
    import { createSlug } from "$lib/utils";
    import { goto } from "$app/navigation";
    import { forceUpdatePackagesStore } from "$lib/stores";

    const modals = getModalStore();

    let name = $state("");
    let slug = $state("");
    let readme = $state("");
    let description = $state("");
    let source = $state<string | undefined>(undefined);
    let issues = $state<string | undefined>(undefined);
    let wiki = $state<string | undefined>(undefined);

    let loading = $state(false);
    let slugError = $state(false);
    let errorMessages = $state<string[]>([]);

    const onChangeName = () => {
        slug = createSlug(name);
        onChangeSlug();
    };

    const onChangeSlug = async () => {
        slugError = !!await getPackage(slug);
    };

    const submit = async (ev: Event) => {
        ev.preventDefault();
        loading = true;
        errorMessages = [];

        if (name == "") {
            errorMessages.push("Name is required");
        }

        if (slug == "") {
            errorMessages.push("Slug is required");
        }

        if (description == "") {
            errorMessages.push("Description is required");
        }

        if (readme == "") {
            errorMessages.push("Readme is required");
        }

        const existing = await getPackage(slug);

        if (existing) {
            errorMessages.push("Slug already exists");
        }

        if (errorMessages.length > 0) {
            loading = false;
            return;
        }

        const data: NewPackage = {
            name,
            slug,
            readme,
            description,
            source: source == "" ? undefined : source,
            issues: issues == "" ? undefined : issues,
            wiki: wiki == "" ? undefined : wiki,
        };

        let res: PackageData | undefined;

        try {
            res = await createPackage(data);

            if (!res) {
                throw new Error("Failed to create package");
            }
        } catch (err) {
            errorMessages.push("Failed to create package");
            loading = false;
            return;
        }

        // Wait for it to be updated or there will be a problem
        await new Promise((res, _rej) => setTimeout(res, 1000));

        await forceUpdatePackagesStore();
        goto(`/p/${res.id}`);

        loading = false;
        modals.close();
    };
</script>

{#if $modals[0]}
    <div class="w-modal bg-secondary-700 relative rounded-lg p-8 shadow-xl">
        <header class="text-2xl font-bold">Create Package</header>

        <form
            class="modal-form mt-4 rounded-md border border-transparent p-4"
            onsubmit={submit}
            class:form-error={errorMessages.length > 0}
        >
            <input
                class="input variant-form-material mb-2 rounded-lg"
                type="text"
                placeholder="Package display name"
                disabled={loading}
                bind:value={name}
                oninput={onChangeName}
            />

            <input
                class="input variant-form-material mb-2 rounded-lg"
                class:mb-0={slugError}
                class:!border-error-500={slugError}
                type="text"
                placeholder="Package slug"
                disabled={loading}
                bind:value={slug}
                oninput={onChangeSlug}
                onchange={onChangeSlug}
            />

            {#if slugError}
            <p class="text-error-500 mb-2">Slug already exists!</p>
            {/if}

            <input
                class="input variant-form-material mb-2 rounded-lg"
                type="text"
                placeholder="Description"
                disabled={loading}
                bind:value={description}
            />

            <input
                class="input variant-form-material mb-2 rounded-lg"
                type="text"
                placeholder="Source code link (optional)"
                disabled={loading}
                bind:value={source}
            />

            <input
                class="input variant-form-material mb-2 rounded-lg"
                type="text"
                placeholder="Issue tracker link (optional)"
                disabled={loading}
                bind:value={issues}
            />

            <input
                class="input variant-form-material mb-2 rounded-lg"
                type="text"
                placeholder="Wiki link (optional)"
                disabled={loading}
                bind:value={wiki}
            />

            <textarea
                class="input variant-form-material mt-2 h-48 w-full rounded-lg"
                placeholder="Readme"
                disabled={loading}
                bind:value={readme}
            ></textarea>
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
                Create
            </button>
        </footer>
    </div>
{/if}

<style lang="postcss">
    :global(input) {
        outline: none !important;
    }
</style>
