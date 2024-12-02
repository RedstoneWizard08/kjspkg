import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";
import { localeGetter } from "./vite/locales";

export default defineConfig({
    plugins: [sveltekit(), localeGetter()],
    clearScreen: false,

    server: {
        port: 4001,
        strictPort: true,
        cors: true,

        hmr: process.env.REDSTONE_IS_DUMB
            ? {
                  clientPort: 443,
                  port: 4001,
                  protocol: "wss",
                  path: "/vite-hmr",
              }
            : {
                  path: "/vite-hmr",
              },
    },
});
