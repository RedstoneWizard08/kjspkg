export interface UserPreferences {
    sortBy: SortMode;
    locale: string;
    theme: string;
    lightMode: boolean;
    compact: boolean;
}

export type SortMode = "" | "name" | "downloads" | "views" | "published" | "updated";
export type LoadingState = "loading" | "ready" | "failed";
