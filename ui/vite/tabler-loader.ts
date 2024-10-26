import fs from "fs";
import path from "path";
import type { SvelteHTMLElements, SVGAttributes } from "svelte/elements";
import type { Plugin } from "vite";

type Attrs = SVGAttributes<SVGSVGElement>;
type IconNodeList = [elementName: keyof SvelteHTMLElements, attrs: Attrs][];

export const tablerIconLoader = (): Plugin => {
    const base = path.dirname(import.meta.dirname);
    const tablerDir = path.join(base, "node_modules", "@tabler", "icons");
    const tablerJson = path.join(tablerDir, "tabler-nodes-outline.json");

    if (!fs.existsSync(tablerJson)) {
        throw new Error(
            "Could not find the nodes JSON file for @tabler/icons! Do you have it installed?",
        );
    }

    const data: Record<string, IconNodeList> = JSON.parse(fs.readFileSync(tablerJson, "utf-8"));

    return {
        name: "tabler-icon-loader",

        resolveId(id) {
            if (id.startsWith("tabler-icon:")) {
                return id;
            }
        },

        load(id) {
            if (id.startsWith("tabler-icon:")) {
                const name = id.slice("tabler-icon:".length);
                const icon = data[name];

                if (!icon) {
                    throw new Error(`Could not find icon with name "${name}"!`);
                }

                return `export default ${JSON.stringify(icon)};`;
            }
        },
    };
};
