export const ssr = false;
export const trailingSlash = "always";

export const load = async (req) => {
    return {
        href: req.url.href,
    };
};
