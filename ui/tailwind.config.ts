import typography from "@tailwindcss/typography";
import forms from "@tailwindcss/forms";
import { skeleton } from "@skeletonlabs/tw-plugin";
import { join } from "path";
import type { Config } from "tailwindcss";
import { defaultTheme } from "./src/themes/kjspkg";
import { lighterTheme } from "./src/themes/kjspkg-lighter";
import { gcatTheme } from "./src/themes/kjspkg-gcat";
import { serenityTheme } from "./src/themes/serenity";

const config = {
    darkMode: "class",
    content: [
        "./src/**/*.{html,js,svelte,ts}",
        join(require.resolve("@skeletonlabs/skeleton"), "../**/*.{html,js,svelte,ts}"),
    ],
    theme: {
        extend: {
            keyframes: {
                "border-blink": {
                    "0%": {
                        "border-color": "transparent",
                    },
                },
                scroll: {
                    from: {
                        transform: "translateX(0)",
                    },
                    to: {
                        transform: "translateX(-100%)",
                    },
                },
            },

            animation: {
                "border-blink": "border-blink 1s steps(2) infinite",
                scroll: "scroll 60s infinite linear",
            },
        },
    },
    plugins: [
        forms,
        typography,
        skeleton({
            themes: {
                custom: [defaultTheme, lighterTheme, gcatTheme, serenityTheme],
                preset: ["wintry", "crimson"],
            },
        }),
    ],
} satisfies Config;

export default config;
