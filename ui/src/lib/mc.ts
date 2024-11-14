import { get, writable } from "svelte/store";

export type GameVersionType = "snapshot" | "release" | "old_alpha" | "old_beta";

export interface LatestVersions {
    release: string;
    snapshot: string;
}

export interface GameVersion {
    id: string;
    type: GameVersionType;
    url: string;
    time: Date;
    releaseTime: Date;
    sha1: string;
    complianceLevel: number;
}

export interface RawVersionManifest {
    latest: LatestVersions;
    versions: GameVersion[];
}

export interface VersionManifest {
    latest: LatestVersions;
    versions: Record<string, GameVersion>;
}

export const MANIFEST_ENDPOINT = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";
export const rawMinecraftVersions = writable<RawVersionManifest | undefined>(undefined);
export const minecraftVersions = writable<VersionManifest | undefined>(undefined);

export const updateMinecraftVersions = async () => {
    const rawData: RawVersionManifest = await (await fetch(MANIFEST_ENDPOINT)).json();
    const versions: Record<string, GameVersion> = {};

    rawMinecraftVersions.set(rawData);

    for (const ver of rawData.versions) {
        versions[ver.id] = ver;
    }

    minecraftVersions.set({
        latest: rawData.latest,
        versions,
    });
};

export const updateVersionsIfNeeded = async () => {
    if (!get(rawMinecraftVersions) || !get(minecraftVersions)) await updateMinecraftVersions();
};
