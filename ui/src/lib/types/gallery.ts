export interface PublicGalleryImage {
    /**
     * The gallery image ID.
     */
    id: number;

    /**
     * The package ID.
     */
    package: number;

    /**
     * The display name of the version.
     */
    name: string;

    /**
     * A URL to access this image with.
     */
    url: string;

    /**
     * An optional markdown-formatted description.
     */
    description?: string;

    /**
     * The order of this image.
     */
    ordering: number;

    /**
     * A stringified {@link Date} for when this image was created.
     */
    created_at: string;

    /**
     * A stringified {@link Date} for when this image was last updated.
     */
    updated_at: string;
}

export interface GalleryImageInit {
    /**
     * The display name of the version.
     */
    name: string;

    /**
     * An optional markdown-formatted description.
     */
    description?: string;

    /**
     * The order of this image. Defaults to -1.
     */
    ordering?: number;
}

export type GalleryImageUpdate = Partial<GalleryImageInit>;
