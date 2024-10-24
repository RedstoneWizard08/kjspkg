import type {
    NewPackage,
    PackageData,
    PackageUpdate,
    PackageVersionInit,
    PackageVersionUpdate,
    User,
} from "$lib/types";
import { getToken } from "./auth";

export const addPackageAuthor = async (
    pkg: string | number,
    author: string | number | User,
): Promise<PackageData | undefined> => {
    const token = getToken();

    if (!token) return undefined;

    try {
        return await (
            await fetch(`/api/v1/packages/${pkg}/authors`, {
                method: "PUT",
                body: "id" in (author as User) ? (author as User).id.toString() : author.toString(),
                headers: {
                    Authorization: `Bearer ${token}`,
                },
            })
        ).json();
    } catch (_err: any) {
        return undefined;
    }
};

export const createPackage = async (data: NewPackage): Promise<PackageData | undefined> => {
    const token = getToken();

    if (!token) return undefined;

    try {
        return await (
            await fetch("/api/v1/packages", {
                method: "PUT",
                body: JSON.stringify(data),
                headers: {
                    Authorization: `Bearer ${token}`,
                    "Content-Type": "application/json",
                },
            })
        ).json();
    } catch (_err: any) {
        return undefined;
    }
};

export const updatePackage = async (
    id: string | number,
    data: PackageUpdate,
): Promise<PackageData | undefined> => {
    const token = getToken();

    if (!token) return undefined;

    try {
        return await (
            await fetch(`/api/v1/packages/${id}`, {
                method: "PATCH",
                body: JSON.stringify(data),
                headers: {
                    Authorization: `Bearer ${token}`,
                    "Content-Type": "application/json",
                },
            })
        ).json();
    } catch (_err: any) {
        return undefined;
    }
};

export const deletePackage = async (id: string | number): Promise<string | undefined> => {
    const token = getToken();

    if (!token) return undefined;

    try {
        return await (
            await fetch(`/api/v1/packages/${id}`, {
                method: "DELETE",
                headers: {
                    Authorization: `Bearer ${token}`,
                },
            })
        ).text();
    } catch (_err: any) {
        return undefined;
    }
};

export const createVersion = async (
    id: string | number,
    data: PackageVersionInit,
    file: File | Blob,
): Promise<string | undefined> => {
    const token = getToken();

    if (!token) return undefined;

    const formData = new FormData();

    formData.set("name", data.name);
    formData.set("kubejs_versions", data.kubejs_versions);
    formData.set("version_number", data.version_number);

    if (data.changelog) formData.set("changelog", data.changelog);

    formData.set("file", file);

    try {
        return await (
            await fetch(`/api/v1/packages/${id}/versions`, {
                method: "PUT",
                body: formData,
                headers: {
                    Authorization: `Bearer ${token}`,
                },
            })
        ).text();
    } catch (_err: any) {
        return undefined;
    }
};

export const updateVersion = async (
    pkg: string | number,
    version: string | number,
    data: PackageVersionUpdate,
): Promise<PackageData | undefined> => {
    const token = getToken();

    if (!token) return undefined;

    try {
        return await (
            await fetch(`/api/v1/packages/${pkg}/versions/${version}`, {
                method: "PATCH",
                body: JSON.stringify(data),
                headers: {
                    Authorization: `Bearer ${token}`,
                    "Content-Type": "application/json",
                },
            })
        ).json();
    } catch (_err: any) {
        return undefined;
    }
};

export const deleteVersion = async (
    pkg: string | number,
    version: string | number,
): Promise<string | undefined> => {
    const token = getToken();

    if (!token) return undefined;

    try {
        return await (
            await fetch(`/api/v1/packages/${pkg}/versions/${version}`, {
                method: "DELETE",
                headers: {
                    Authorization: `Bearer ${token}`,
                },
            })
        ).text();
    } catch (_err: any) {
        return undefined;
    }
};
