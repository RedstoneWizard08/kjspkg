/// <reference path="./vite-locales.d.ts" />

import { browser } from "$app/environment";
import { addMessages, init } from "svelte-i18n";
import { allLocales, localeData } from "@locales";

const defaultLocale = "en-US";

for (const locale of allLocales) {
    addMessages(locale, localeData[locale]);
}

// addMessages("en_US", en_US);
// addMessages("de_DE", de_DE);
// addMessages("it_IT", it_IT);
// addMessages("ru_RU", ru_RU);
// addMessages("zh_TW", zh_TW);
// addMessages("en_alpha", en_alpha);

init({
    fallbackLocale: defaultLocale,
    initialLocale: browser ? window.navigator.language : defaultLocale,
});
