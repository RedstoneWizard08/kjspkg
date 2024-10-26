// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces

declare module "tabler-icon:*" {
    type Attrs = SVGAttributes<SVGSVGElement>;
    type IconNodeList = [elementName: keyof SvelteHTMLElements, attrs: Attrs][];
    const nodes: IconNodeList;

    export default nodes;
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
