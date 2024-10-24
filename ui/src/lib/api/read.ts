import type { PackageData, PackageVersion, User } from "$lib/types";

export const getUser = async (id: string | number): Promise<User | undefined> => {
    try {
        return await (await fetch(`/api/v1/users/${id}`)).json();
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

export const getPackages = async (): Promise<PackageData[] | undefined> => {
    try {
        return await (await fetch("/api/v1/packages")).json();
    } catch (_err: any) {
        return undefined;
    }
};

export const getPackage = async (id: string | number): Promise<PackageData | undefined> => {
    try {
        return await (await fetch(`/api/v1/packages/${id}`)).json();
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
): Promise<PackageVersion[] | undefined> => {
    try {
        return await (await fetch(`/api/v1/packages/${pkg}/versions`)).json();
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
