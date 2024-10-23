import { get, writable } from "svelte/store";
import type { UserPreferences, Vec2 } from "./types";
import { browser } from "$app/environment";
import { localStorageStore } from "@skeletonlabs/skeleton";
import { locales } from "svelte-i18n";

export const currentScrollPosition = writable<Vec2>({ x: 0, y: 0 });

export const userPreferencesStore = localStorageStore<UserPreferences>("preferences", {
    sortBy: "name",
    locale: browser && get(locales).includes(navigator.language) ? navigator.language : "en-US",
    theme: "kjspkg",
    lightMode: false,
    compact: false,
    alreadyVisited: true,
});
