import { writable } from "svelte/store";

export type ContextMenuProps = {
    initiator: "left" | "right" | "left-right";
    items: ContextMenuItem[];
};
export type ContextMenuItem =
    | {
          type: "ITEM";
          label: string;
          icon?: ConstructorOfATypedSvelteComponent;
          action: () => any | Promise<any>;
      }
    | { type: "SEPARATOR"; header?: string };
// | { type: 'TOGGLE'; label: string; icon?: ConstructorOfATypedSvelteComponent; action: () => void, checked: boolean };

export const contextMenu = (node: HTMLElement, initialProps: ContextMenuProps) => {
    let props = initialProps;

    async function callback(e: MouseEvent) {
        let x = e.clientX + 5;
        let y = e.clientY + 5;

        contextMenuStore.set({
            x,
            y,
            invisible: true,
            ...props,
        });

        await new Promise((r) => setTimeout(r, 1));
        const menu = document.querySelector("#GLOBAL-ctxm") as HTMLDivElement;

        // Get dimensions of menu
        let menuX = menu.offsetWidth;
        let menuY = menu.offsetHeight;

        // Get viewport size
        let viewportX = window.innerWidth;
        let viewportY = window.innerHeight;

        // menu should be away from viewport border.
        if (menuX + x > viewportX - 20) x = viewportX - menuX - 20;
        if (menuY + y > viewportY - 20) y = viewportY - menuY - 20;

        contextMenuStore.set({
            x,
            y,
            invisible: false,
            ...props,
        });
    }

    node.addEventListener("contextmenu", (e) => {
        if (!(props.initiator == "right" || props.initiator == "left-right")) return;
        setTimeout(() => callback(e));
        e.preventDefault();
    });
    node.addEventListener("click", (e) => {
        if (!(props.initiator == "left" || props.initiator == "left-right")) return;
        setTimeout(() => callback(e));
    });

    return {
        update(newProps: ContextMenuProps) {
            props = newProps;
        },
    };
};

export const contextMenuStore = writable<
    (ContextMenuProps & { x: number; y: number; invisible: boolean }) | undefined
>();
