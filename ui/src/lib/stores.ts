import { get, writable } from "svelte/store";
import type { User, PackageData, UserPreferences, Vec2, SortMode } from "./types";
import { browser } from "$app/environment";
import { locales } from "svelte-i18n";
import { getCurrentUser, getPackages } from "$api";
import { persisted } from "svelte-persisted-store";

export const currentScrollPosition = writable<Vec2>({ x: 0, y: 0 });
export const currentSearchStore = writable<string>("");
export const packagesStore = writable<PackageData[]>([]);
export const filteredStore = writable<PackageData[]>([]);
export const user = writable<User | undefined>(undefined);
export const currentPackage = writable<PackageData | undefined>(undefined);

export const userPreferencesStore = persisted<UserPreferences>("preferences", {
    sortBy: "name",
    locale: browser && get(locales).includes(navigator.language) ? navigator.language : "en-US",
    theme: "kjspkg",
    lightMode: false,
    compact: false,
});

export const forceUpdatePackagesStore = async () => {
    packagesStore.set((await getPackages()) ?? []);
};

export const updatePackagesStore = async () => {
    if (get(packagesStore).length == 0) packagesStore.set((await getPackages()) ?? []);

    return get(packagesStore).length != 0;
};

export const updateFilteredPackages = () => {
    filteredStore.set(sortPackages(get(packagesStore), get(userPreferencesStore).sortBy, true));
};

export const sortPackages = (packagesIn: PackageData[], sortBy: SortMode, search = false) => {
    let packages = packagesIn.map((v) => ({ ...v }));

    if (search) {
        const query = get(currentSearchStore).toLowerCase();

        if (query != "") {
            packages = packages.filter(
                (v) =>
                    v.name.toLowerCase().includes(query) ||
                    v.description.toLowerCase().includes(query) ||
                    !!v.authors.find((u) => u.username.toLowerCase().includes(query)),
            );
        }
    }

    const alphabetic = packages.sort((a, b) => {
        return a.name == b.name ? 0 : a.name < b.name ? -1 : 1;
    });

    let result = packages;

    switch (sortBy) {
        case "":
            result = packages;
            break;
        case "name":
            result = alphabetic;
            break;
        case "downloads":
            result = alphabetic.sort((a, b) =>
                a.downloads == b.downloads ? 0 : a.downloads < b.downloads ? 1 : -1,
            );
            break;
        case "views":
            result = alphabetic.sort((a, b) =>
                a.views == b.views ? 0 : a.views < b.views ? 1 : -1,
            );
            break;
        case "published":
            result = alphabetic.sort((a, b) =>
                new Date(a.created_at) == new Date(b.created_at)
                    ? 0
                    : new Date(a.created_at) < new Date(b.created_at)
                      ? 1
                      : -1,
            );
            break;
        case "updated":
            result = alphabetic.sort((a, b) =>
                new Date(a.updated_at) == new Date(b.updated_at)
                    ? 0
                    : new Date(a.updated_at) < new Date(b.updated_at)
                      ? 1
                      : -1,
            );
            break;
    }

    return result;
};

export const updateUserStore = async () => {
    if (get(user)) return;

    user.set(await getCurrentUser());
};

export const updateTheme = () => {
    if (!browser) return;

    if (get(userPreferencesStore).lightMode) {
        document.documentElement.classList.remove("dark");
    } else {
        document.documentElement.classList.add("dark");
    }
};

currentSearchStore.subscribe(updateFilteredPackages);
userPreferencesStore.subscribe(updateFilteredPackages);
userPreferencesStore.subscribe(updateTheme);
