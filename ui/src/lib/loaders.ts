import { get, writable } from "svelte/store";
import { siteConfig } from "./config";

export interface ModLoader {
    id: string;
    name: boolean;
}

export const modLoaders = writable<ModLoader[] | undefined>(undefined);

export const updateModLoadersIfNeeded = async () => {
    if (!get(modLoaders)) {
        modLoaders.set(await siteConfig.loaderFetcher());
    }
};
