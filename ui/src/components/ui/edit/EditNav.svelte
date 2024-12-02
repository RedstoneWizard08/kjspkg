<script lang="ts">
    import EditButton from "./EditButton.svelte";

    interface Props {
        id: string;
    }

    const { id }: Props = $props();
</script>

<div class="card flex w-[20%] flex-col gap-2 p-4">
    <EditButton {id} route="/p/[id]" icon="tabler:arrow-left" text="Back" />
    <EditButton {id} route="/p/[id]/edit" icon="tabler:settings" text="General" />
    <EditButton {id} route="/p/[id]/edit/tags" icon="tabler:tags" text="Tags" />

    <EditButton
        {id}
        route="/p/[id]/edit/description"
        icon="tabler:file-description"
        text="Description"
    />

    <EditButton {id} route="/p/[id]/edit/members" icon="tabler:users" text="Members" />
    <EditButton {id} route="/p/[id]/edit/gallery" icon="tabler:library-photo" text="Gallery" />

    <EditButton
        {id}
        route="/p/[id]/edit/versions"
        icon="tabler:versions"
        text="Versions"
        extraMatches={["/p/[id]/edit/versions/create", "/p/[id]/edit/versions/edit/[version]"]}
    />
</div>
