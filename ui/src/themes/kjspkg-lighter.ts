import type { CustomThemeConfig } from "@skeletonlabs/tw-plugin";

export const lighterTheme: CustomThemeConfig = {
    name: "kjspkg-lighter",
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
        "--on-secondary": "var(--color-surface-50)",
        "--on-tertiary": "0 0 0",
        "--on-success": "var(--color-surface-900)",
        "--on-warning": "0 0 0",
        "--on-error": "var(--color-surface-50)",
        "--on-surface": "var(--color-surface-50)",
        // =~= Theme Colors  =~=
        // primary | #c37ee5
        "--color-primary-50": "246 236 251", // #f6ecfb
        "--color-primary-100": "243 229 250", // #f3e5fa
        "--color-primary-200": "240 223 249", // #f0dff9
        "--color-primary-300": "231 203 245", // #e7cbf5
        "--color-primary-400": "213 165 237", // #d5a5ed
        "--color-primary-500": "195 126 229", // #c37ee5
        "--color-primary-600": "176 113 206", // #b071ce
        "--color-primary-700": "146 95 172", // #925fac
        "--color-primary-800": "117 76 137", // #754c89
        "--color-primary-900": "96 62 112", // #603e70
        // secondary | #4e355a
        "--color-secondary-50": "228 225 230", // #e4e1e6
        "--color-secondary-100": "220 215 222", // #dcd7de
        "--color-secondary-200": "211 205 214", // #d3cdd6
        "--color-secondary-300": "184 174 189", // #b8aebd
        "--color-secondary-400": "131 114 140", // #83728c
        "--color-secondary-500": "78 53 90", // #4e355a
        "--color-secondary-600": "70 48 81", // #463051
        "--color-secondary-700": "59 40 68", // #3b2844
        "--color-secondary-800": "47 32 54", // #2f2036
        "--color-secondary-900": "38 26 44", // #261a2c
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
        // success | #8ee190
        "--color-success-50": "238 251 238", // #eefbee
        "--color-success-100": "232 249 233", // #e8f9e9
        "--color-success-200": "227 248 227", // #e3f8e3
        "--color-success-300": "210 243 211", // #d2f3d3
        "--color-success-400": "176 234 177", // #b0eab1
        "--color-success-500": "142 225 144", // #8ee190
        "--color-success-600": "128 203 130", // #80cb82
        "--color-success-700": "107 169 108", // #6ba96c
        "--color-success-800": "85 135 86", // #558756
        "--color-success-900": "70 110 71", // #466e47
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
        // error | #b11061
        "--color-error-50": "243 219 231", // #f3dbe7
        "--color-error-100": "239 207 223", // #efcfdf
        "--color-error-200": "236 195 216", // #ecc3d8
        "--color-error-300": "224 159 192", // #e09fc0
        "--color-error-400": "200 88 144", // #c85890
        "--color-error-500": "177 16 97", // #b11061
        "--color-error-600": "159 14 87", // #9f0e57
        "--color-error-700": "133 12 73", // #850c49
        "--color-error-800": "106 10 58", // #6a0a3a
        "--color-error-900": "87 8 48", // #570830
        // surface | #633e79
        "--color-surface-50": "232 226 235", // #e8e2eb
        "--color-surface-100": "224 216 228", // #e0d8e4
        "--color-surface-200": "216 207 222", // #d8cfde
        "--color-surface-300": "193 178 201", // #c1b2c9
        "--color-surface-400": "146 120 161", // #9278a1
        "--color-surface-500": "99 62 121", // #633e79
        "--color-surface-600": "89 56 109", // #59386d
        "--color-surface-700": "74 47 91", // #4a2f5b
        "--color-surface-800": "59 37 73", // #3b2549
        "--color-surface-900": "49 30 59", // #311e3b
    },
};
