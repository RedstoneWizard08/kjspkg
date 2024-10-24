import { error, redirect } from "@sveltejs/kit";
import { base } from "$app/paths";
import type { PageLoad } from "./$types";

export const load: PageLoad = (data) => {
    const id = new URL(data.url).searchParams.get("id");
    if (!id) error(404, "Not Found");
    redirect(301, base + "/p/" + id);
};
