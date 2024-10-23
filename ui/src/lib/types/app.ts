export interface UserPreferences {
    sortBy: SortMode;
    locale: string;
    theme: string;
    lightMode: boolean;
    compact: boolean;
    alreadyVisited: boolean;
}

export type SortMode = "" | "name" | "downloads" | "views";
