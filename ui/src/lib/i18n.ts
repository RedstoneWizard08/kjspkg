import { browser } from "$app/environment";
import { addMessages, init } from "svelte-i18n";
import en_US from "$locales/en_US.json";
import de_DE from "$locales/de_DE.json";
import it_IT from "$locales/it_IT.json";
import ru_RU from "$locales/ru_RU.json";
import zh_TW from "$locales/zh_TW.json";
import en_alpha from "$locales/en_alpha.json";

const defaultLocale = "en_US";

addMessages("en_US", en_US);
addMessages("de_DE", de_DE);
addMessages("it_IT", it_IT);
addMessages("ru_RU", ru_RU);
addMessages("zh_TW", zh_TW);
addMessages("en_alpha", en_alpha);

init({
    fallbackLocale: defaultLocale,
    initialLocale: browser ? window.navigator.language : defaultLocale,
});
