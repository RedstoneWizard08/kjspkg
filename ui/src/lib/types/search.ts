import type { PackageData } from "./pkg";

export type SortMode = "none" | "name" | "downloads" | "published" | "updated";
export type SortDirection = "asc" | "desc";

export interface Pagination {
    page: number;
    per_page: number;
    results: number;
    total: number;
    pages: number;
}

export interface SearchResults {
    pagination: Pagination;
    results: PackageData[];
}
