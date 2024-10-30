<script lang="ts">
    import { _ } from "svelte-i18n";
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
        slugError = !!(await getPackage(slug));
    };

    const submit = async (ev: Event) => {
        ev.preventDefault();
        loading = true;
        errorMessages = [];

        if (name == "") {
            errorMessages.push($_("modal.create_package.error.name"));
        }

        if (slug == "") {
            errorMessages.push($_("modal.create_package.error.slug"));
        }

        if (description == "") {
            errorMessages.push($_("modal.create_package.error.description"));
        }

        if (readme == "") {
            errorMessages.push($_("modal.create_package.error.readme"));
        }

        const existing = await getPackage(slug);

        if (existing) {
            errorMessages.push($_("modal.create_package.error.slug_exists"));
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
                throw new Error($_("modal.create_package.error.api"));
            }
        } catch (err) {
            errorMessages.push($_("modal.create_package.error.api"));
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
    <div class="w-modal relative rounded-lg bg-secondary-700 p-8 shadow-xl">
        <header class="text-2xl font-bold">{$_("modal.create_package.title")}</header>

        <form
            class="modal-form mt-4 rounded-md border border-transparent p-4"
            onsubmit={submit}
            class:form-error={errorMessages.length > 0}
        >
            <input
                class="input variant-form-material mb-2 rounded-lg"
                type="text"
                placeholder={$_("modal.create_package.placeholder.name")}
                disabled={loading}
                bind:value={name}
                oninput={onChangeName}
            />

            <input
                class="input variant-form-material mb-2 rounded-lg"
                class:mb-0={slugError}
                class:!border-error-500={slugError}
                type="text"
                placeholder={$_("modal.create_package.placeholder.slug")}
                disabled={loading}
                bind:value={slug}
                oninput={onChangeSlug}
                onchange={onChangeSlug}
            />

            {#if slugError}
                <p class="mb-2 text-error-500">{$_("modal.create_package.error.slug_exists")}</p>
            {/if}

            <input
                class="input variant-form-material mb-2 rounded-lg"
                type="text"
                placeholder={$_("modal.create_package.placeholder.description")}
                disabled={loading}
                bind:value={description}
            />

            <input
                class="input variant-form-material mb-2 rounded-lg"
                type="text"
                placeholder={$_("modal.create_package.placeholder.source")}
                disabled={loading}
                bind:value={source}
            />

            <input
                class="input variant-form-material mb-2 rounded-lg"
                type="text"
                placeholder={$_("modal.create_package.placeholder.issues")}
                disabled={loading}
                bind:value={issues}
            />

            <input
                class="input variant-form-material mb-2 rounded-lg"
                type="text"
                placeholder={$_("modal.create_package.placeholder.wiki")}
                disabled={loading}
                bind:value={wiki}
            />

            <textarea
                class="input variant-form-material mt-2 h-48 w-full rounded-lg"
                placeholder={$_("modal.create_package.placeholder.readme")}
                disabled={loading}
                bind:value={readme}
            ></textarea>
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
                {$_("action.create")}
            </button>
        </footer>
    </div>
{/if}

<style lang="postcss">
    :global(input) {
        outline: none !important;
    }
</style>
