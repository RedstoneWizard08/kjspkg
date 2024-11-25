export interface PackageVersion {
    /**
     * This version's ID.
     */
    id: number;

    /**
     * The ID of the package this version belongs to.
     */
    package: number;

    /**
     * The display name of this version.
     */
    name: string;

    /**
     * This version's version number.
     */
    version_number: string;

    /**
     * The changelog for this version.
     * Can be empty/undefined.
     */
    changelog?: string;

    /**
     * A list of modloaders this version works on.
     */
    loaders: string[];

    /**
     * A list of game versions this version works on.
     */
    game_versions: string[];

    /**
     * A stringified {@link Date} for when this version was created.
     */
    created_at: string;

    /**
     * A stringified {@link Date} for when this version was updated.
     */
    updated_at: string;

    /**
     * The number of downloads this package has.
     */
    downloads: number;

    /**
     * The file name.
     */
    file_name: string;
}

export interface PackageVersionInit {
    /**
     * The display name of this version.
     */
    name: string;

    /**
     * This version's version number.
     */
    version_number: string;

    /**
     * The changelog for this version.
     * Can be empty/undefined.
     */
    changelog?: string;

    /**
     * A list of modloaders this version works on.
     */
    loaders: string[];

    /**
     * A list of game versions this version works on.
     */
    game_versions: string[];
}

export type PackageVersionUpdate = Partial<PackageVersionInit>;
