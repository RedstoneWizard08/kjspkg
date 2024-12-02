declare module "tabler-icon:*" {
    type Attrs = SVGAttributes<SVGSVGElement>;
    type IconNodeList = [elementName: keyof SvelteHTMLElements, attrs: Attrs][];
    const nodes: IconNodeList;

    export default nodes;
}
