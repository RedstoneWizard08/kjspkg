import type { User } from "./user";

export interface Package {
    /**
     * The ID of the package.
     */
    id: number;

    /**
     * The package name.
     */
    name: string;

    /**
     * The slug (URL identifier) of the package.
     */
    slug: string;

    /**
     * The content of this package's README file.
     */
    readme: string;

    /**
     * This package's description.
     */
    description: string;

    /**
     * An optional link to this package's source code.
     */
    source?: string;

    /**
     * An optional link to this package's issue tracker.
     */
    issues?: string;

    /**
     * An optional link to this package's wiki.
     */
    wiki?: string;

    /**
     * A stringified {@link Date} for when this version was created.
     */
    created_at: string;

    /**
     * A stringified {@link Date} for when this version was updated.
     */
    updated_at: string;

    /**
     * The amount of views this package gets.
     */
    views: number;
}

export interface PackageUpdate {
    /**
     * The package name.
     */
    name: string;

    /**
     * The content of this package's README file.
     */
    readme: string;

    /**
     * This package's description.
     */
    description: string;

    /**
     * Whether this package supports Forge (LexForge).
     */
    supports_forge: boolean;

    /**
     * Whether this package supports Fabric.
     */
    supports_fabric: boolean;

    /**
     * Whether this package supports Quilt.
     */
    supports_quilt: boolean;

    /**
     * Whether this package suppoorts NeoForge.
     */
    supports_neoforge: boolean;
}

export interface NewPackage {
    /**
     * The name of the package.
     */
    name: string;

    /**
     * The requested slug for the package.
     */
    slug: string;

    /**
     * The package's README file contents.
     */
    readme: string;

    /**
     * A short description of the package.
     */
    description: string;

    /**
     * Whether the package supports Forge (LexForge).
     * Defaults to `false`.
     */
    supports_forge?: boolean;

    /**
     * Whether the package supports Fabric.
     * Defaults to `false`.
     */
    supports_fabric?: boolean;

    /**
     * Whether the package supports Quilt.
     * Defaults to `false`.
     */
    supports_quilt?: boolean;

    /**
     * Whether the package supports NeoForge.
     * Defaults to `false`.
     */
    supports_neoforge?: boolean;
}

export interface PackageData extends Package {
    /**
     * The amount of downloads all of this package's versions have, combined.
     */
    downloads: number;

    /**
     * A list of users that have access to modify this package (a.k.a. authors).
     */
    authors: User[];
}
