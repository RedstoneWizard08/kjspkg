import type { CustomThemeConfig } from "@skeletonlabs/tw-plugin";

export const modhostTheme: CustomThemeConfig = {
    name: "modhost",

    properties: {
        // =~= Theme Properties =~=
        "--theme-font-family-base": `Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji'`,
        "--theme-font-family-heading": `Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji'`,
        "--theme-font-color-base": "0 0 0",
        "--theme-font-color-dark": "var(--color-surface-50)",
        "--theme-rounded-base": "9999px",
        "--theme-rounded-container": "8px",
        "--theme-border-base": "2px",
        // =~= Theme On-X Colors =~=
        "--on-primary": "var(--color-surface-900)",
        "--on-secondary": "var(--color-surface-50)",
        "--on-tertiary": "0 0 0",
        "--on-success": "var(--color-surface-900)",
        "--on-warning": "0 0 0",
        "--on-error": "var(--color-surface-50)",
        "--on-surface": "var(--color-surface-50)",
        // =~= Theme Colors  =~=
        // primary | #0ad6ff
        "--color-primary-50": "218 249 255", // #daf9ff
        "--color-primary-100": "206 247 255", // #cef7ff
        "--color-primary-200": "194 245 255", // #c2f5ff
        "--color-primary-300": "157 239 255", // #9defff
        "--color-primary-400": "84 226 255", // #54e2ff
        "--color-primary-500": "10 214 255", // #0ad6ff
        "--color-primary-600": "9 193 230", // #09c1e6
        "--color-primary-700": "8 161 191", // #08a1bf
        "--color-primary-800": "6 128 153", // #068099
        "--color-primary-900": "5 105 125", // #05697d
        // secondary | #413caa
        "--color-secondary-50": "227 226 242", // #e3e2f2
        "--color-secondary-100": "217 216 238", // #d9d8ee
        "--color-secondary-200": "208 206 234", // #d0ceea
        "--color-secondary-300": "179 177 221", // #b3b1dd
        "--color-secondary-400": "122 119 196", // #7a77c4
        "--color-secondary-500": "65 60 170", // #413caa
        "--color-secondary-600": "59 54 153", // #3b3699
        "--color-secondary-700": "49 45 128", // #312d80
        "--color-secondary-800": "39 36 102", // #272466
        "--color-secondary-900": "32 29 83", // #201d53
        // tertiary | #1e5a76
        "--color-tertiary-50": "221 230 234", // #dde6ea
        "--color-tertiary-100": "210 222 228", // #d2dee4
        "--color-tertiary-200": "199 214 221", // #c7d6dd
        "--color-tertiary-300": "165 189 200", // #a5bdc8
        "--color-tertiary-400": "98 140 159", // #628c9f
        "--color-tertiary-500": "30 90 118", // #1e5a76
        "--color-tertiary-600": "27 81 106", // #1b516a
        "--color-tertiary-700": "23 68 89", // #174459
        "--color-tertiary-800": "18 54 71", // #123647
        "--color-tertiary-900": "15 44 58", // #0f2c3a
        // success | #3bd86a
        "--color-success-50": "226 249 233", // #e2f9e9
        "--color-success-100": "216 247 225", // #d8f7e1
        "--color-success-200": "206 245 218", // #cef5da
        "--color-success-300": "177 239 195", // #b1efc3
        "--color-success-400": "118 228 151", // #76e497
        "--color-success-500": "59 216 106", // #3bd86a
        "--color-success-600": "53 194 95", // #35c25f
        "--color-success-700": "44 162 80", // #2ca250
        "--color-success-800": "35 130 64", // #238240
        "--color-success-900": "29 106 52", // #1d6a34
        // warning | #EAB308
        "--color-warning-50": "252 244 218", // #fcf4da
        "--color-warning-100": "251 240 206", // #fbf0ce
        "--color-warning-200": "250 236 193", // #faecc1
        "--color-warning-300": "247 225 156", // #f7e19c
        "--color-warning-400": "240 202 82", // #f0ca52
        "--color-warning-500": "234 179 8", // #EAB308
        "--color-warning-600": "211 161 7", // #d3a107
        "--color-warning-700": "176 134 6", // #b08606
        "--color-warning-800": "140 107 5", // #8c6b05
        "--color-warning-900": "115 88 4", // #735804
        // error | #be2754
        "--color-error-50": "245 223 229", // #f5dfe5
        "--color-error-100": "242 212 221", // #f2d4dd
        "--color-error-200": "239 201 212", // #efc9d4
        "--color-error-300": "229 169 187", // #e5a9bb
        "--color-error-400": "210 104 135", // #d26887
        "--color-error-500": "190 39 84", // #be2754
        "--color-error-600": "171 35 76", // #ab234c
        "--color-error-700": "143 29 63", // #8f1d3f
        "--color-error-800": "114 23 50", // #721732
        "--color-error-900": "93 19 41", // #5d1329
        // surface | #333b52
        "--color-surface-50": "224 226 229", // #e0e2e5
        "--color-surface-100": "214 216 220", // #d6d8dc
        "--color-surface-200": "204 206 212", // #ccced4
        "--color-surface-300": "173 177 186", // #adb1ba
        "--color-surface-400": "112 118 134", // #707686
        "--color-surface-500": "51 59 82", // #333b52
        "--color-surface-600": "46 53 74", // #2e354a
        "--color-surface-700": "38 44 62", // #262c3e
        "--color-surface-800": "31 35 49", // #1f2331
        "--color-surface-900": "25 29 40", // #191d28
    },
};
