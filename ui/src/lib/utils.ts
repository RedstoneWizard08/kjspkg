import markdownit from "markdown-it";
import { full as emoji } from "markdown-it-emoji";
import type { ModLoader, PackageData, PackageVersion, SortMode } from "./types";
import { get } from "svelte/store";
import { _ } from "svelte-i18n";

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
            return get(_)("loader.forge");
        case "fabric":
            return get(_)("loader.fabric");
        case "quilt":
            return get(_)("loader.quilt");
        case "neoforge":
            return get(_)("loader.neoforge");
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

export const capText = (text: string, len: number) => {
    if (text.length < len) return text;

    return text.substring(0, len - 3) + "...";
};

export const splitToRows = <T>(data: T[], rows: number): T[][] => {
    const out = [];
    const cols = Math.floor(data.length / rows);

    for (let i = 0; i < rows; i++) {
        const items = i == rows - 1 ? data.length - cols * i : cols;
        const start = Math.max(0, cols * i);

        out.push(data.slice(start, Math.min(start + items, data.length - 1)));
    }

    return out;
};
