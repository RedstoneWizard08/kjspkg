import { writable, get } from "svelte/store";

export const licensesCache = writable<string[] | undefined>(undefined);
export const licensesUrl = "https://licenses.opendefinition.org/licenses/groups/all.json";

export const getLicenses = async () => {
    if (!get(licensesCache))
        licensesCache.set(Object.keys(await (await fetch(licensesUrl)).json()));

    return get(licensesCache)!;
};
