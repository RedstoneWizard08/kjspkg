import type { Sort, SortMode } from "./search";

export interface UserPreferences {
    sortBy: Sort;
    sortDir: SortMode;
    locale: string;
    theme: string;
    lightMode: boolean;
    compact: boolean;
}

export type LoadingState = "loading" | "ready" | "failed";
