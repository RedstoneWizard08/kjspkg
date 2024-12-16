import type { PackageData, PackageVersion, User } from "$lib/types";
import type { PublicGalleryImage } from "$lib/types/gallery";
import type { SearchResults } from "$lib/types/search";

export const getUser = async (id: string | number): Promise<User | undefined> => {
    try {
        return await (await fetch(`/api/v1/users/${id}`)).json();
    } catch (_err: any) {
        return undefined;
    }
};

export const searchUsers = async (query: string): Promise<User[] | undefined> => {
    try {
        return await (await fetch(`/api/v1/users/search?q=${query}`)).json();
    } catch (_err: any) {
        return undefined;
    }
};

export const getUserPackages = async (id: string | number): Promise<PackageData[] | undefined> => {
    try {
        return await (await fetch(`/api/v1/users/${id}/packages`)).json();
    } catch (_err: any) {
        return undefined;
    }
};

export const searchPackages = async (
    q?: string,
    page = 1,
    perPage = 30,
): Promise<SearchResults | undefined> => {
    let query = "";

    if (q) {
        query = "?q=" + encodeURIComponent(q);
    }

    if (query.startsWith("?")) query += `&page=${page}&per_page=${perPage}`;
    else query += `?page=${page}&per_page=${perPage}`;

    try {
        return await (await fetch(`/api/v1/packages/search${query}`)).json();
    } catch (_err: any) {
        return undefined;
    }
};

export const getPackage = async (
    id: string | number,
    fetcher = fetch,
): Promise<PackageData | undefined> => {
    try {
        return await (await fetcher(`/api/v1/packages/${id}`)).json();
    } catch (_err: any) {
        return undefined;
    }
};

export const getPackageAuthors = async (pkg: string | number): Promise<User[] | undefined> => {
    try {
        return await (await fetch(`/api/v1/packages/${pkg}/authors`)).json();
    } catch (_err: any) {
        return undefined;
    }
};

export const getPackageVersions = async (
    pkg: string | number,
    fetcher = fetch,
): Promise<PackageVersion[] | undefined> => {
    try {
        return await (await fetcher(`/api/v1/packages/${pkg}/versions`)).json();
    } catch (_err: any) {
        return undefined;
    }
};

export const getPackageVersion = async (
    pkg: string | number,
    version: string | number,
): Promise<PackageVersion | undefined> => {
    try {
        return await (await fetch(`/api/v1/packages/${pkg}/versions/${version}`)).json();
    } catch (_err: any) {
        return undefined;
    }
};

export const getPackageVersionFile = async (
    pkg: string | number,
    version: string | number,
): Promise<ArrayBuffer | undefined> => {
    try {
        return await (
            await fetch(`/api/v1/packages/${pkg}/versions/${version}/download`)
        ).arrayBuffer();
    } catch (_err: any) {
        return undefined;
    }
};

export const getPackageGallery = async (
    pkg: string | number,
): Promise<PublicGalleryImage[] | undefined> => {
    try {
        return await (await fetch(`/api/v1/packages/${pkg}/gallery`)).json();
    } catch (_err: any) {
        return undefined;
    }
};

export const getGalleryImage = async (
    pkg: string | number,
    img: string | number,
): Promise<PublicGalleryImage | undefined> => {
    try {
        return await (await fetch(`/api/v1/packages/${pkg}/gallery/${img}`)).json();
    } catch (_err: any) {
        return undefined;
    }
};
