import { browser } from "$app/environment";
import { init, register } from "svelte-i18n";

const defaultLocale = "en_US";

register("en_US", () => import("$locales/en_US.json"));
register("de_DE", () => import("$locales/de_DE.json"));
register("it_IT", () => import("$locales/it_IT.json"));
register("ru_RU", () => import("$locales/ru_RU.json"));
register("zh_TW", () => import("$locales/zh_TW.json"));
register("en_alpha", () => import("$locales/en_alpha.json"));

init({
    fallbackLocale: defaultLocale,
    initialLocale: browser ? window.navigator.language : defaultLocale,
});
