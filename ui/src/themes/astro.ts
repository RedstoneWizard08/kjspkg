import type { CustomThemeConfig } from "@skeletonlabs/tw-plugin";

export const astroTheme: CustomThemeConfig = {
    name: "astro",

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
        "--on-tertiary": "var(--color-surface-50)",
        "--on-success": "var(--color-surface-900)",
        "--on-warning": "0 0 0",
        "--on-error": "var(--color-surface-50)",
        "--on-surface": "var(--color-surface-50)",
        // =~= Theme Colors  =~=
        // primary | #129af0
        "--color-primary-50": "219 240 253", // #dbf0fd
        "--color-primary-100": "208 235 252", // #d0ebfc
        "--color-primary-200": "196 230 251", // #c4e6fb
        "--color-primary-300": "160 215 249", // #a0d7f9
        "--color-primary-400": "89 184 245", // #59b8f5
        "--color-primary-500": "18 154 240", // #129af0
        "--color-primary-600": "16 139 216", // #108bd8
        "--color-primary-700": "14 116 180", // #0e74b4
        "--color-primary-800": "11 92 144", // #0b5c90
        "--color-primary-900": "9 75 118", // #094b76
        // secondary | #8500ad
        "--color-secondary-50": "237 217 243", // #edd9f3
        "--color-secondary-100": "231 204 239", // #e7ccef
        "--color-secondary-200": "225 191 235", // #e1bfeb
        "--color-secondary-300": "206 153 222", // #ce99de
        "--color-secondary-400": "170 77 198", // #aa4dc6
        "--color-secondary-500": "133 0 173", // #8500ad
        "--color-secondary-600": "120 0 156", // #78009c
        "--color-secondary-700": "100 0 130", // #640082
        "--color-secondary-800": "80 0 104", // #500068
        "--color-secondary-900": "65 0 85", // #410055
        // tertiary | #323776
        "--color-tertiary-50": "224 225 234", // #e0e1ea
        "--color-tertiary-100": "214 215 228", // #d6d7e4
        "--color-tertiary-200": "204 205 221", // #cccddd
        "--color-tertiary-300": "173 175 200", // #adafc8
        "--color-tertiary-400": "112 115 159", // #70739f
        "--color-tertiary-500": "50 55 118", // #323776
        "--color-tertiary-600": "45 50 106", // #2d326a
        "--color-tertiary-700": "38 41 89", // #262959
        "--color-tertiary-800": "30 33 71", // #1e2147
        "--color-tertiary-900": "25 27 58", // #191b3a
        // success | #6FE757
        "--color-success-50": "233 251 230", // #e9fbe6
        "--color-success-100": "226 250 221", // #e2fadd
        "--color-success-200": "219 249 213", // #dbf9d5
        "--color-success-300": "197 245 188", // #c5f5bc
        "--color-success-400": "154 238 137", // #9aee89
        "--color-success-500": "111 231 87", // #6FE757
        "--color-success-600": "100 208 78", // #64d04e
        "--color-success-700": "83 173 65", // #53ad41
        "--color-success-800": "67 139 52", // #438b34
        "--color-success-900": "54 113 43", // #36712b
        // warning | #E6CE33
        "--color-warning-50": "251 248 224", // #fbf8e0
        "--color-warning-100": "250 245 214", // #faf5d6
        "--color-warning-200": "249 243 204", // #f9f3cc
        "--color-warning-300": "245 235 173", // #f5ebad
        "--color-warning-400": "238 221 112", // #eedd70
        "--color-warning-500": "230 206 51", // #E6CE33
        "--color-warning-600": "207 185 46", // #cfb92e
        "--color-warning-700": "173 155 38", // #ad9b26
        "--color-warning-800": "138 124 31", // #8a7c1f
        "--color-warning-900": "113 101 25", // #716519
        // error | #C74638
        "--color-error-50": "247 227 225", // #f7e3e1
        "--color-error-100": "244 218 215", // #f4dad7
        "--color-error-200": "241 209 205", // #f1d1cd
        "--color-error-300": "233 181 175", // #e9b5af
        "--color-error-400": "216 126 116", // #d87e74
        "--color-error-500": "199 70 56", // #C74638
        "--color-error-600": "179 63 50", // #b33f32
        "--color-error-700": "149 53 42", // #95352a
        "--color-error-800": "119 42 34", // #772a22
        "--color-error-900": "98 34 27", // #62221b
        // surface | #332d5c
        "--color-surface-50": "224 224 231", // #e0e0e7
        "--color-surface-100": "214 213 222", // #d6d5de
        "--color-surface-200": "204 203 214", // #cccbd6
        "--color-surface-300": "173 171 190", // #adabbe
        "--color-surface-400": "112 108 141", // #706c8d
        "--color-surface-500": "51 45 92", // #332d5c
        "--color-surface-600": "46 41 83", // #2e2953
        "--color-surface-700": "38 34 69", // #262245
        "--color-surface-800": "31 27 55", // #1f1b37
        "--color-surface-900": "25 22 45", // #19162d
    },
};
