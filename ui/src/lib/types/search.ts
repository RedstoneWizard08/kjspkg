import type { PackageData } from "./pkg";

export interface Facets {
    game_versions: string[];
    loaders: string[];
    published: [number, number];
    updated: [number, number];
    downloads: [number, number];
}

export type Facet<K extends keyof Facets> = [K, Facets[K]];
export type Sort = "none" | "name" | "downloads" | "published" | "updated";
export type SortMode = "asc" | "desc";

export interface SearchResults {
    page: number;
    pages: number;
    hits: number;
    total: number;
    results: PackageData[];
}
