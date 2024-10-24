import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";

export default defineConfig({
    plugins: [sveltekit()],
    clearScreen: false,

    server: {
        port: 4001,
        strictPort: true,
        cors: true,

        hmr: {
            clientPort: 443,
            port: 4001,
            protocol: "wss",
            path: "/vite-hmr",
        },
    },
});
