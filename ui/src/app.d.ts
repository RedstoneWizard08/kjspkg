declare module "$env/static/public" {
    export const PUBLIC_APP: string;
    export const PUBLIC_TAGLINE: string;
    export const PUBLIC_SHOW_BETA: string;
    export const PUBLIC_PKG_TYPE: string;
    export const PUBLIC_DEFAULT_THEME: string;
    export const PUBLIC_PKG_FILE_FORMATS: string;
    export const PUBLIC_GAME_BETA_NAME: string;
    export const PUBLIC_THEME_COLOR: string;
}

declare global {
    namespace App {
        // interface Error {}
        // interface Locals {}
        // interface PageData {}
        // interface PageState {}
        // interface Platform {}
    }
}

export {};
