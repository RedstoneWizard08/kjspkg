/// <reference path="./tabler.d.ts" />

import type { SvelteHTMLElements, SVGAttributes } from "svelte/elements";

export const icons = {
    "alert-triangle": () => import("tabler-icon:alert-triangle"),
    "brand-github": () => import("tabler-icon:brand-github"),
    "color-swatch": () => import("tabler-icon:color-swatch"),
    "device-floppy": () => import("tabler-icon:device-floppy"),
    "clear-all": () => import("tabler-icon:clear-all"),
    "layout-dashboard": () => import("tabler-icon:layout-dashboard"),
    "arrow-left": () => import("tabler-icon:arrow-left"),
    "login-2": () => import("tabler-icon:login-2"),
    "caret-down": () => import("tabler-icon:caret-down"),
    "menu-2": () => import("tabler-icon:menu-2"),
    "loader-2": () => import("tabler-icon:loader-2"),
    "file-description": () => import("tabler-icon:file-description"),
    "library-photo": () => import("tabler-icon:library-photo"),
    user: () => import("tabler-icon:user"),
    search: () => import("tabler-icon:search"),
    check: () => import("tabler-icon:check"),
    world: () => import("tabler-icon:world"),
    download: () => import("tabler-icon:download"),
    pencil: () => import("tabler-icon:pencil"),
    plus: () => import("tabler-icon:plus"),
    trash: () => import("tabler-icon:trash"),
    upload: () => import("tabler-icon:upload"),
    login: () => import("tabler-icon:login"),
    sun: () => import("tabler-icon:sun"),
    moon: () => import("tabler-icon:moon"),
    home: () => import("tabler-icon:home"),
    settings: () => import("tabler-icon:settings"),
    tags: () => import("tabler-icon:tags"),
    license: () => import("tabler-icon:license"),
    users: () => import("tabler-icon:users"),
    versions: () => import("tabler-icon:versions"),
};

export type IconName = keyof typeof icons;
export type IconType = "outline" | "filled";
export type Attrs = SVGAttributes<SVGSVGElement>;
export type IconNodeList = [elementName: keyof SvelteHTMLElements, attrs: Attrs][];
