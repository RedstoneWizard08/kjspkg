<script lang="ts">
    import EditContainer from "$components/ui/edit/EditContainer.svelte";
    import { onMount, type Snippet } from "svelte";
    import { page } from "$app/stores";
    import { currentPackage, editLoadingState, user } from "$lib/stores";
    import { getPackage } from "$api";
    import { beforeNavigate, goto } from "$app/navigation";

    const id = $derived($page.params.id);
    const ok = $derived(!!$currentPackage?.authors.find((v) => v.id == $user?.id));

    const editRoutes = [
        "/p/[id]/edit",
        "/p/[id]/edit/description",
        "/p/[id]/edit/gallery",
        "/p/[id]/edit/members",
        "/p/[id]/edit/tags",
        "/p/[id]/edit/versions",
        "/p/[id]/edit/versions/create",
        "/p/[id]/edit/versions/edit/[version]",
    ];

    onMount(async () => {
        $currentPackage = await getPackage(id);

        if ($currentPackage) {
            $editLoadingState = "ready";
        } else {
            $editLoadingState = "failed";
        }

        setTimeout(() => {
            if (!ok) {
                goto(`/p/${id}`); // TODO: This is a really bad way of doing this
            }
        }, 500);
    });

    beforeNavigate(({ to }) => {
        if (!editRoutes.includes(to?.route.id ?? "")) {
            $currentPackage = undefined;
            $editLoadingState = "loading";
        }
    });

    const { data, children }: { data: any; children: Snippet } = $props();
</script>

{#if ok}
    <EditContainer>
        {#key data.href}
            {@render children?.()}
        {/key}
    </EditContainer>
{/if}
