import type { CustomThemeConfig } from "@skeletonlabs/tw-plugin";

export const serenityTheme: CustomThemeConfig = {
    name: "serenity",
    properties: {
        // =~= Theme Properties =~=
        "--theme-font-family-base": `Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji'`,
        "--theme-font-family-heading": `Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji'`,
        "--theme-font-color-base": "0 0 0",
        "--theme-font-color-dark": "var(--color-surface-50)",
        "--theme-rounded-base": "9999px",
        "--theme-rounded-container": "12px",
        "--theme-border-base": "2px",
        // =~= Theme On-X Colors =~=
        "--on-primary": "var(--color-surface-900)",
        "--on-secondary": "var(--color-surface-900)",
        "--on-tertiary": "0 0 0",
        "--on-success": "var(--color-surface-900)",
        "--on-warning": "0 0 0",
        "--on-error": "var(--color-surface-900)",
        "--on-surface": "var(--color-surface-50)",
        // =~= Theme Colors  =~=
        // primary | #fab387
        "--color-primary-50": "254 244 237", // #fef4ed
        "--color-primary-100": "254 240 231", // #fef0e7
        "--color-primary-200": "254 236 225", // #feece1
        "--color-primary-300": "253 225 207", // #fde1cf
        "--color-primary-400": "252 202 171", // #fccaab
        "--color-primary-500": "250 179 135", // #fab387
        "--color-primary-600": "225 161 122", // #e1a17a
        "--color-primary-700": "188 134 101", // #bc8665
        "--color-primary-800": "150 107 81", // #966b51
        "--color-primary-900": "123 88 66", // #7b5842
        // secondary | #f5e0dc
        "--color-secondary-50": "254 250 250", // #fefafa
        "--color-secondary-100": "253 249 248", // #fdf9f8
        "--color-secondary-200": "253 247 246", // #fdf7f6
        "--color-secondary-300": "251 243 241", // #fbf3f1
        "--color-secondary-400": "248 233 231", // #f8e9e7
        "--color-secondary-500": "245 224 220", // #f5e0dc
        "--color-secondary-600": "221 202 198", // #ddcac6
        "--color-secondary-700": "184 168 165", // #b8a8a5
        "--color-secondary-800": "147 134 132", // #938684
        "--color-secondary-900": "120 110 108", // #786e6c
        // tertiary | #000000
        "--color-tertiary-50": "217 217 217", // #d9d9d9
        "--color-tertiary-100": "204 204 204", // #cccccc
        "--color-tertiary-200": "191 191 191", // #bfbfbf
        "--color-tertiary-300": "153 153 153", // #999999
        "--color-tertiary-400": "77 77 77", // #4d4d4d
        "--color-tertiary-500": "0 0 0", // #000000
        "--color-tertiary-600": "0 0 0", // #000000
        "--color-tertiary-700": "0 0 0", // #000000
        "--color-tertiary-800": "0 0 0", // #000000
        "--color-tertiary-900": "0 0 0", // #000000
        // success | #a6e3a1
        "--color-success-50": "242 251 241", // #f2fbf1
        "--color-success-100": "237 249 236", // #edf9ec
        "--color-success-200": "233 248 232", // #e9f8e8
        "--color-success-300": "219 244 217", // #dbf4d9
        "--color-success-400": "193 235 189", // #c1ebbd
        "--color-success-500": "166 227 161", // #a6e3a1
        "--color-success-600": "149 204 145", // #95cc91
        "--color-success-700": "125 170 121", // #7daa79
        "--color-success-800": "100 136 97", // #648861
        "--color-success-900": "81 111 79", // #516f4f
        // warning | #000000
        "--color-warning-50": "217 217 217", // #d9d9d9
        "--color-warning-100": "204 204 204", // #cccccc
        "--color-warning-200": "191 191 191", // #bfbfbf
        "--color-warning-300": "153 153 153", // #999999
        "--color-warning-400": "77 77 77", // #4d4d4d
        "--color-warning-500": "0 0 0", // #000000
        "--color-warning-600": "0 0 0", // #000000
        "--color-warning-700": "0 0 0", // #000000
        "--color-warning-800": "0 0 0", // #000000
        "--color-warning-900": "0 0 0", // #000000
        // error | #f38ba8
        "--color-error-50": "253 238 242", // #fdeef2
        "--color-error-100": "253 232 238", // #fde8ee
        "--color-error-200": "252 226 233", // #fce2e9
        "--color-error-300": "250 209 220", // #fad1dc
        "--color-error-400": "247 174 194", // #f7aec2
        "--color-error-500": "243 139 168", // #f38ba8
        "--color-error-600": "219 125 151", // #db7d97
        "--color-error-700": "182 104 126", // #b6687e
        "--color-error-800": "146 83 101", // #925365
        "--color-error-900": "119 68 82", // #774452
        // surface | #454d4f
        "--color-surface-50": "227 228 229", // #e3e4e5
        "--color-surface-100": "218 219 220", // #dadbdc
        "--color-surface-200": "209 211 211", // #d1d3d3
        "--color-surface-300": "181 184 185", // #b5b8b9
        "--color-surface-400": "125 130 132", // #7d8284
        "--color-surface-500": "69 77 79", // #454d4f
        "--color-surface-600": "62 69 71", // #3e4547
        "--color-surface-700": "52 58 59", // #343a3b
        "--color-surface-800": "41 46 47", // #292e2f
        "--color-surface-900": "34 38 39", // #222627
    },
};
