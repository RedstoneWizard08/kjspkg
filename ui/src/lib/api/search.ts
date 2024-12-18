import type { Facet, SearchResults, Sort, SortMode } from "$lib/types/search";

export const serializeFacets = (facets: Facet<any>[]) => {
    return `[${facets.map((facet) => `["${facet[0]}", ["${facet[1]}"]]`).join(",")}]`;
};

export const searchPackages = async (
    q?: string,
    facets: Facet<any>[] = [],
    sort?: Sort,
    sortMode?: SortMode,
    page = 1,
    perPage = 30,
): Promise<SearchResults | undefined> => {
    const query: Record<string, string> = {};

    if (q) query["q"] = encodeURIComponent(q);
    if (sort) query["sort"] = sort;
    if (sortMode) query["dir"] = sortMode;

    query["page"] = page.toString();
    query["per_page"] = perPage.toString();
    query["filters"] = serializeFacets(facets);

    let queryStr = "";

    for (const [k, v] of Object.entries(query)) {
        if (queryStr.startsWith("?")) {
            queryStr += `&${k}=${v}`;
        } else {
            queryStr += `?${k}=${v}`;
        }
    }

    try {
        return await (await fetch(`/api/v1/packages/search${queryStr}`)).json();
    } catch (_err: any) {
        return undefined;
    }
};
