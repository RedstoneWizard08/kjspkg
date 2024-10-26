import markdownit from "markdown-it";
import { full as emoji } from "markdown-it-emoji";
import type { ModLoader, PackageData, PackageVersion, SortMode } from "./types";

const md = markdownit({
    html: false,
    xhtmlOut: false,
    breaks: false,
    langPrefix: "hljs-",
    linkify: true,
    typographer: true,
    quotes: "“”‘’",
    highlight: (/*str, lang*/) => "",
}).use(emoji);

export const allLoaders: ModLoader[] = ["forge", "fabric", "quilt", "neoforge"];
export const markdownInline = (str: string): string => md.renderInline(str);
export const markdown = (str: string): string => md.render(str);
export const removeBase = (target: string, base: string) => target.replace(base, "");

export const guessSortMode = (input: string): SortMode => {
    if (["", "name", "downloads", "views"].includes(input)) {
        return input as SortMode;
    } else {
        return "";
    }
};

export const getLoaders = (versions: PackageVersion[]) => {
    const data: string[] = [];

    for (const version of versions) {
        for (const item of version.loaders) {
            if (!data.includes(item)) data.push(item);
        }
    }

    return data;
};

export const getMinecraft = (versions: PackageVersion[]) => {
    const data: string[] = [];

    for (const version of versions) {
        for (const item of version.minecraft) {
            if (!data.includes(item)) data.push(item);
        }
    }

    return data;
};

export const getKubeJS = (versions: PackageVersion[]) => {
    const data: string[] = [];

    for (const version of versions) {
        for (const item of version.kubejs) {
            if (!data.includes(item)) data.push(item);
        }
    }

    return data;
};

export const fixLoaderName = (name: string) => {
    switch (name.toLowerCase()) {
        case "forge":
            return "Forge";
        case "fabric":
            return "Fabric";
        case "quilt":
            return "Quilt";
        case "neoforge":
            return "NeoForge";
        default:
            return name;
    }
};

export const formatDate = (date: Date) => {
    return new Intl.DateTimeFormat(undefined, {
        year: "numeric",
        month: "numeric",
        day: "numeric",
        hour: "numeric",
        minute: "numeric",
        second: "numeric",
        hour12: true,
    }).format(date);
};

export const createSlug = (input: string) => {
    const slugRegex = /[^a-z0-9_-]/gm;

    return input.toLowerCase().replace(slugRegex, "-");
};
