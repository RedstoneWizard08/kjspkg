import { get, writable } from "svelte/store";
import { siteConfig } from "./config";

export interface GameVersion {
    id: string;
    beta: boolean;
}

export const gameVersions = writable<GameVersion[] | undefined>(undefined);

export const updateGameVersionsIfNeeded = async () => {
    if (!get(gameVersions)) {
        gameVersions.set(await siteConfig.versionFetcher());
    }
};
