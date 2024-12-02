import fs from "fs";
import path from "path";
import type { Plugin } from "vite";

interface LocaleDictionary {
    [key: string]: LocaleDictionary | string | Array<string | LocaleDictionary> | null;
}

type LocalesDictionary = {
    [key: string]: LocaleDictionary;
};

const locales = [
    "af-ZA",
    "ar-SA",
    "ca-ES",
    "cs-CZ",
    "da-DK",
    "de-DE",
    "el-GR",
    "en-alpha",
    "en-UD",
    "en-US",
    "es-CO",
    "fi-FI",
    "fr-FR",
    "he-IL",
    "hu-HU",
    "it-IT",
    "ja-JP",
    "ko-KR",
    "nl-NL",
    "no-NO",
    "pl-PL",
    "pt-BR",
    "pt-PT",
    "ro-RO",
    "ru-RU",
    "sr-SP",
    "sv-SE",
    "tr-TR",
    "uk-UA",
    "vi-VN",
    "zh-CN",
    "zh-TW",
];

export const localeGetter = (): Plugin => {
    const base = path.dirname(import.meta.dirname);
    const langDir = path.join(base, "src", "locales");

    return {
        name: "locale-getter",

        resolveId: (id) => (id == "@locales" ? "@locales?id=" + crypto.randomUUID() : undefined),

        load: (id) => {
            if (id.startsWith("@locales")) {
                const data: Record<string, LocaleDictionary> = {};

                for (const locale of locales) {
                    data[locale] = JSON.parse(
                        fs.readFileSync(path.join(langDir, `${locale}.json`), "utf-8"),
                    );
                }

                const scriptText = `export const allLocales = JSON.parse(\`${JSON.stringify(locales)}\`);
export const localeData = JSON.parse(\`${JSON.stringify(data)}\`);`;

                return scriptText;
            }
        },
    };
};
