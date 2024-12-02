declare module "@locales" {
    interface LocaleDictionary {
        [key: string]: LocaleDictionary | string | Array<string | LocaleDictionary> | null;
    }

    export const allLocales: string[];
    export const localeData: Record<string, LocaleDictionary>;
}
