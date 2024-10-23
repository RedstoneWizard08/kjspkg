const typography = require("@tailwindcss/typography");
const forms = require("@tailwindcss/forms");
import { skeleton } from "@skeletonlabs/tw-plugin";

import { join } from "path";
import type { Config } from "tailwindcss";

import { ThemeKjspkg } from "./src/themes/kjspkg";
import { ThemeKjspkgLighter } from "./src/themes/kjspkg-lighter";
import { ThemeKjspkgGcat } from "./src/themes/kjspkg-gcat";
import { ThemeSerenity } from "./src/themes/serenity";

const config = {
    darkMode: "class",
    content: [
        "./src/**/*.{html,js,svelte,ts}",
        join(require.resolve("@skeletonlabs/skeleton"), "../**/*.{html,js,svelte,ts}"),
    ],
    theme: {
        extend: {},
    },
    plugins: [
        forms,
        typography,
        skeleton({
            themes: {
                custom: [ThemeKjspkg, ThemeKjspkgLighter, ThemeKjspkgGcat, ThemeSerenity],
                preset: ["wintry", "crimson"],
            },
        }),
    ],
} satisfies Config;

export default config;
