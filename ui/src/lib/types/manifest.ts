export type ModLoader = "forge" | "fabric" | "quilt" | "neoforge";

export interface PackageManifest {
    /**
     * The package's name.
     */
    name: string;

    /**
     * The package's authors.
     */
    authors: string[];

    /**
     * The package version.
     */
    version: string;

    /**
     * A short description of the package.
     */
    description: string;

    /**
     * A list of game versions this package works on.
     * Can be `["all"]`.
     */
    game_versions: string[];

    /**
     * A list of loaders this package works on.
     * Must NOT be empty.
     */
    loaders: ModLoader[];

    /**
     * A list packages this one depends on.
     * Can be empty.
     */
    dependencies: string[];

    /**
     * A list of packages this one is incompatible with.
     * Can be empty.
     */
    incompatibilities: string[];
}
