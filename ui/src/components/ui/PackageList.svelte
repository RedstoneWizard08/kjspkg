<script lang="ts">
    import { _ } from "svelte-i18n";
    import { base } from "$app/paths";
    import { page } from "$app/stores";
    import { formatDate } from "$lib/utils";
    import type { SearchResults } from "$lib/types/search";
    import PackageCard from "./PackageCard.svelte";

    interface Props {
        customHeight?: number;
        compact?: boolean;
        showAvatar?: boolean;
        showDetails?: boolean;
        showName?: boolean;
        select?: (id: number) => void | Promise<void>;
        packages: SearchResults;
    }

    const {
        customHeight,
        compact = false,
        showName = true,
        showDetails = true,
        showAvatar = true,
        select,
        packages,
    }: Props = $props();
</script>

{#each packages.results as pkg, _ (pkg)}
    <PackageCard {pkg} {customHeight} {compact} {showName} {showDetails} {showAvatar} {select} />
{/each}
