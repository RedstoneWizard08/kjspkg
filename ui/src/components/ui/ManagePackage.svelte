<script lang="ts">
    import type { PackageVersion } from "$lib/types";
    import { clipboard, getToastStore, popup } from "@skeletonlabs/skeleton";

    interface Props {
        name: string;
        version?: PackageVersion;
        link?: string;
    }

    const { name, version, link }: Props = $props();
    const toastStore = getToastStore();
    const options = ["install", "remove", "update", null, "pkg"];
</script>

{#each options as o}
    {#if o}
        <button
            class="code pt-1 text-left hover:brightness-110 active:scale-95"
            use:popup={{ event: "click", placement: "right", target: "copy/" + o }}
            use:clipboard={`kjspkg ${o} ${name}${version ? "@" + version.version_number : ""}`}
            onclick={() =>
                toastStore.trigger({
                    message: "Copied to clipboard!",
                    hideDismiss: true,
                    timeout: 1000,
                    background: "variant-filled-success",
                })}
        >
            kjspkg {o}
            {name}{version ? "@" + version.version_number : ""}
        </button>
    {:else}
        <hr />
    {/if}
{/each}
