<script lang="ts">
    import { _ } from "svelte-i18n";
    import { page } from "$app/stores";
    import { onMount } from "svelte";
    import { addPackageAuthor, getPackage, getUser, removePackageAuthor } from "$api";
    import { currentPackage, editSaving } from "$lib/stores";
    import Icon from "@iconify/svelte";
    import { getToastStore } from "@skeletonlabs/skeleton";
    import type { User } from "$lib/types";

    const id = $derived($page.params.id);
    const toasts = getToastStore();

    let person = $state("");
    let checking = $state(false);
    let authors = $state<User[]>([]);
    let displayAuthors = $state<User[]>([]);
    let newAuthors = $state<number[]>([]);
    let removedAuthors = $state<number[]>([]);

    onMount(() => {
        if (!$currentPackage) return;

        authors = [...$currentPackage.authors];
        displayAuthors = [...authors];
    });

    const save = async () => {
        $editSaving = true;

        let newPkg = $currentPackage;

        for (const user of $currentPackage?.authors ?? []) {
            if (!authors.find((v) => v.id == user.id)) {
                newPkg = await removePackageAuthor(id, user.id);
            }
        }

        for (const user of authors) {
            if (!newPkg?.authors.find((v) => v.id == user.id)) {
                newPkg = await addPackageAuthor(id, user.id);
            }
        }

        $currentPackage = await getPackage(id);

        authors = $currentPackage?.authors ?? authors;
        newAuthors = [];
        removedAuthors = [];
        displayAuthors = [...authors];

        $editSaving = false;
    };

    const removeMember = (userId: number) => {
        return async (ev: Event) => {
            ev.preventDefault();
            ev.stopPropagation();

            if ($editSaving) {
                toasts.trigger({
                    message: `Saving in progress, try again later.`,
                    hideDismiss: true,
                    timeout: 5000,
                    background: "variant-filled-error",
                });

                return;
            }

            if (authors.length <= 1) {
                toasts.trigger({
                    message: `A project must have one or more members!`,
                    hideDismiss: true,
                    timeout: 5000,
                    background: "variant-filled-error",
                });

                return;
            }

            authors = authors.filter((v) => v.id != userId);
            removedAuthors.push(userId);

            if (newAuthors.includes(userId)) {
                newAuthors = newAuthors.filter((v) => v != userId);
                displayAuthors = displayAuthors.filter((v) => v.id != userId);
            }
        };
    };

    const addPerson = async () => {
        if ($editSaving) {
            toasts.trigger({
                message: `Saving in progress, try again later.`,
                hideDismiss: true,
                timeout: 5000,
                background: "variant-filled-error",
            });

            return;
        }

        if (checking) {
            toasts.trigger({
                message: `A user is already being fetched, try again later.`,
                hideDismiss: true,
                timeout: 5000,
                background: "variant-filled-error",
            });

            return;
        }

        checking = true;

        try {
            const user = await getUser(person);

            // trigger the catch block
            if (!user) throw new Error("No user found!");

            authors.push(user);
            newAuthors.push(user.id);

            if (!displayAuthors.find((v) => v.id == user.id)) displayAuthors.push(user);

            if (removedAuthors.includes(user.id)) {
                removedAuthors = removedAuthors.filter((v) => v != user.id);
            }
        } catch (_err) {
            toasts.trigger({
                message: `Could not find user with name ${person}!`,
                hideDismiss: true,
                timeout: 5000,
                background: "variant-filled-error",
            });
        }

        person = "";
        checking = false;
    };
</script>

<div class="card variant-soft-secondary w-full space-y-2 p-4">
    <p class="mb-4 flex flex-row items-center justify-start text-primary-500">
        <Icon icon="tabler:users" height="24" class="mr-2" />
        Project Members
    </p>

    {#each displayAuthors as author}
        <a
            class="card flex flex-row items-center justify-between p-2 hover:variant-soft-primary"
            href="/u/{author.username}"
        >
            <div
                class="flex flex-row items-center justify-start"
                class:!text-success-500={newAuthors.includes(author.id)}
                class:!font-bold={newAuthors.includes(author.id) ||
                    removedAuthors.includes(author.id)}
                class:!line-through={removedAuthors.includes(author.id)}
                class:!text-error-500={removedAuthors.includes(author.id)}
            >
                <img
                    src="https://avatars.githubusercontent.com/u/{author.github_id}"
                    alt="author's profile afirst child cssvatar"
                    class="my-auto mr-2 aspect-square h-8 rounded-token"
                />
                {author.username}
            </div>

            <button
                type="button"
                class="variant-soft-error btn transition-all hover:variant-filled-error"
                onclick={removeMember(author.id)}
            >
                <Icon icon="tabler:trash" height="24" />
            </button>
        </a>
    {/each}
</div>

<div class="card variant-soft-secondary w-full space-y-2 p-4">
    <p class="mb-2 flex flex-row items-center justify-start text-primary-500">
        <Icon icon="tabler:plus" height="24" class="mr-2" />
        Add People
    </p>

    <input
        type="text"
        placeholder="Type a username..."
        class="input rounded-md"
        bind:value={person}
        disabled={checking}
    />

    <button
        type="button"
        class="variant-ghost-secondary btn flex flex-row items-center justify-center rounded-lg transition-all hover:variant-soft-primary"
        onclick={addPerson}
        disabled={checking}
    >
        {#if checking}
            <Icon icon="tabler:loader-2" height="24" class="mr-2 animate-spin" />
        {:else}
            <Icon icon="tabler:plus" height="24" class="mr-2" />
        {/if}

        Add
    </button>
</div>

<button
    type="button"
    class="variant-filled-primary btn mt-2 flex flex-row items-center justify-center rounded-lg"
    onclick={save}
>
    <Icon icon="tabler:device-floppy" height="24" class="mr-2" />
    Save
</button>
