// @ts-nocheck

import type { SvelteHTMLElements, SVGAttributes } from "svelte/elements";
import alertTriangleData from "tabler-icon:alert-triangle";
import brandGithubData from "tabler-icon:brand-github";
import userData from "tabler-icon:user";
import searchData from "tabler-icon:search";
import checkData from "tabler-icon:check";
import colorSwatchData from "tabler-icon:color-swatch";
import worldData from "tabler-icon:world";
import downloadData from "tabler-icon:download";
import pencilData from "tabler-icon:pencil";
import deviceFloppyData from "tabler-icon:device-floppy";
import clearAllData from "tabler-icon:clear-all";
import layoutDashboardData from "tabler-icon:layout-dashboard";
import plusData from "tabler-icon:plus";
import trashData from "tabler-icon:trash";
import uploadData from "tabler-icon:upload";
import arrowLeftData from "tabler-icon:arrow-left";
import loginData from "tabler-icon:login";
import login2Data from "tabler-icon:login-2";
import caretDownData from "tabler-icon:caret-down";
import sunData from "tabler-icon:sun";
import moonData from "tabler-icon:moon";

export const icons = {
    // alert-triangle, brand-github, user, search, check, color-swatch, world, download, pencil, device-floppy, clear-all, layout-dashboard
    "alert-triangle": alertTriangleData,
    "brand-github": brandGithubData,
    user: userData,
    search: searchData,
    check: checkData,
    "color-swatch": colorSwatchData,
    world: worldData,
    download: downloadData,
    pencil: pencilData,
    "device-floppy": deviceFloppyData,
    "clear-all": clearAllData,
    "layout-dashboard": layoutDashboardData,
    plus: plusData,
    trash: trashData,
    upload: uploadData,
    "arrow-left": arrowLeftData,
    "login-2": login2Data,
    login: loginData,
    "caret-down": caretDownData,
    sun: sunData,
    moon: moonData,
};

export type IconName = keyof typeof icons;
export type IconType = "outline" | "filled";
export type Attrs = SVGAttributes<SVGSVGElement>;
export type IconNodeList = [elementName: keyof SvelteHTMLElements, attrs: Attrs][];
