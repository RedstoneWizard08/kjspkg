import { get, writable } from "svelte/store";
import type { User, PackageData, UserPreferences, Vec2 } from "./types";
import { browser } from "$app/environment";
import { localStorageStore } from "@skeletonlabs/skeleton";
import { locales } from "svelte-i18n";
import { getCurrentUser, getPackages } from "$api";

export const currentScrollPosition = writable<Vec2>({ x: 0, y: 0 });
export const currentSearchStore = writable<string>("");
export const packagesStore = writable<PackageData[]>([]);
export const filteredStore = writable<PackageData[]>([]);
export const user = writable<User | undefined>(undefined);

export const userPreferencesStore = localStorageStore<UserPreferences>("preferences", {
    sortBy: "name",
    locale: browser && get(locales).includes(navigator.language) ? navigator.language : "en-US",
    theme: "kjspkg",
    lightMode: false,
    compact: false,
});

export const updatePackagesStore = async () => {
    if (get(packagesStore).length == 0) packagesStore.set((await getPackages()) ?? []);

    return get(packagesStore).length != 0;
};

export const updateFilteredPackages = () => {
    let packages = get(packagesStore);
    const query = get(currentSearchStore).toLowerCase();

    if (query != "") {
        packages = packages.filter(
            (v) =>
                v.name.toLowerCase().includes(query) ||
                v.description.toLowerCase().includes(query) ||
                !!v.authors.find((u) => u.username.toLowerCase().includes(query)),
        );
    }

    const alphabetic = packages.sort((a, b) => {
        return a.name == b.name ? 0 : a.name < b.name ? -1 : 1;
    });

    switch (get(userPreferencesStore).sortBy) {
        case "":
            filteredStore.set(packages);
            break;
        case "name":
            filteredStore.set(alphabetic);
            break;
        case "downloads":
            filteredStore.set(
                alphabetic.sort((a, b) =>
                    a.downloads == b.downloads ? 0 : a.downloads < b.downloads ? 1 : -1,
                ),
            );
            break;
        case "views":
            filteredStore.set(
                alphabetic.sort((a, b) => (a.views == b.views ? 0 : a.views < b.views ? 1 : -1)),
            );
            break;
    }
};

export const updateUserStore = async () => {
    if (get(user)) return;

    user.set(await getCurrentUser());
};

currentSearchStore.subscribe(updateFilteredPackages);
userPreferencesStore.subscribe(updateFilteredPackages);
