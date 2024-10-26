<script lang="ts">
    import { icons, type Attrs, type IconName, type IconNodeList, type IconType } from "$lib/icons";
    import type { Snippet } from "svelte";

    interface Props {
        name: IconName;
        type?: IconType;
        color?: string;
        size?: number | string;
        stroke?: number | string;
        class?: string;
        children?: Snippet;
        [key: string]: any;
    }

    const {
        name,
        type = "outline",
        color = "currentColor",
        size = 24,
        stroke = 2,
        class: klass,
        children,
        ...rest
    }: Props = $props();

    const iconNode: IconNodeList = icons[name];

    const defaultAttributes: Record<"outline" | "filled", Attrs> = {
        outline: {
            xmlns: "http://www.w3.org/2000/svg",
            width: 24,
            height: 24,
            viewBox: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            "stroke-width": 2,
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
        },
        filled: {
            xmlns: "http://www.w3.org/2000/svg",
            width: 24,
            height: 24,
            viewBox: "0 0 24 24",
            fill: "currentColor",
            stroke: "none",
        },
    };
</script>

<svg
    {...defaultAttributes[type]}
    {...rest}
    width={size}
    height={size}
    class={`tabler-icon tabler-icon-${name} ${klass ?? ""}`}
    {...type === "filled"
        ? {
              fill: color,
          }
        : {
              "stroke-width": stroke,
              stroke: color,
          }}
>
    {#each iconNode as [tag, attrs]}
        <svelte:element this={tag} {...attrs} />
    {/each}

    {@render children?.()}
</svg>
