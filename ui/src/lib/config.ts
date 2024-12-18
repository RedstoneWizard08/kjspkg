/// <reference path="../app.d.ts" />

import type { GameVersion } from "./versions";
import {
    PUBLIC_APP,
    PUBLIC_DEFAULT_THEME,
    PUBLIC_GAME_BETA_NAME,
    PUBLIC_PKG_FILE_FORMATS,
    PUBLIC_PKG_TYPE,
    PUBLIC_SHOW_BETA,
    PUBLIC_TAGLINE,
    PUBLIC_THEME_COLOR,
} from "$env/static/public";
import type { ModLoader } from "./loaders";

export interface SiteConfig {
    siteName: string;
    tagline: string;
    showBeta: boolean;
    type: "mods" | "packages";
    defaultTheme: string;
    packageFileFormats: string[];
    betaName: "beta" | "snapshot";
    themeColor: string;
    versionFetcher: () => Promise<GameVersion[]>;
    loaderFetcher: () => Promise<ModLoader[]>;
}

export const siteConfig: SiteConfig = {
    siteName: PUBLIC_APP,
    tagline: PUBLIC_TAGLINE,
    showBeta: PUBLIC_SHOW_BETA == "true",
    type: PUBLIC_PKG_TYPE as "mods" | "packages",
    defaultTheme: PUBLIC_DEFAULT_THEME,
    packageFileFormats: PUBLIC_PKG_FILE_FORMATS.split(","),
    betaName: PUBLIC_GAME_BETA_NAME as "beta" | "snapshot",
    themeColor: PUBLIC_THEME_COLOR,
    versionFetcher: async () => await (await fetch("/api/v1/meta/game_versions")).json(),
    loaderFetcher: async () => await (await fetch("/api/v1/meta/loaders")).json(),
};
