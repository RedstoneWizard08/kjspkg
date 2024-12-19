import { get, writable } from "svelte/store";
import {
    type User,
    type PackageData,
    type UserPreferences,
    type Vec2,
    type LoadingState,
    type Tag,
} from "./types";
import { browser } from "$app/environment";
import { locales } from "svelte-i18n";
import { getCurrentUser, getTags, searchPackages } from "$api";
import { persisted } from "svelte-persisted-store";
import { siteConfig } from "./config";
import type { SearchResults } from "./types/search";

export const emptySearchResults: SearchResults = {
    page: 1,
    pages: 0,
    hits: 0,
    total: 0,
    results: [],
};

export const currentScrollPosition = writable<Vec2>({ x: 0, y: 0 });
export const currentSearchStore = writable<string>("");
export const packagesStore = writable<SearchResults>(emptySearchResults);
export const user = writable<User | undefined>(undefined);
export const currentPackage = writable<PackageData | undefined>(undefined);
export const editSaving = writable<boolean>(false);
export const editLoadingState = writable<LoadingState>("loading");
export const tagsStore = writable<Tag[]>([]);

export const userPreferencesStore = persisted<UserPreferences>("preferences", {
    sortBy: "none",
    sortDir: "asc",
    locale: browser && get(locales).includes(navigator.language) ? navigator.language : "en-US",
    theme: siteConfig.defaultTheme,
    lightMode: false,
    compact: false,
});

export const forceUpdatePackagesStore = async () => {
    packagesStore.set((await searchPackages()) ?? emptySearchResults);
};

export const updatePackagesStore = async () => {
    if (get(packagesStore).hits == 0)
        packagesStore.set((await searchPackages()) ?? emptySearchResults);

    return get(packagesStore).hits != 0;
};

export const updateUserStore = async () => {
    if (get(user)) return;

    user.set(await getCurrentUser());
};

export const updateTagsStore = async () => {
    tagsStore.set(await getTags());
};

export const updateTheme = () => {
    if (!browser) return;

    if (get(userPreferencesStore).lightMode) {
        document.documentElement.classList.remove("dark");
    } else {
        document.documentElement.classList.add("dark");
    }
};

userPreferencesStore.subscribe(updateTheme);
