import { goto } from "$app/navigation";
import type { User } from "$lib/types";
import { persisted } from "svelte-persisted-store";
import { get } from "svelte/store";

const tokenStore = persisted<string | undefined>("kjspkg-auth-token", undefined);

export const setToken = (token?: string) => tokenStore.set(token);
export const getToken = () => get(tokenStore);
export const isLoggedIn = () => !!getToken();

export const beginLogin = (redirect: string) =>
    goto(
        redirect
            ? `/api/v1/auth/github/login?redirect_uri=${redirect}`
            : "/api/v1/auth/github/login",
    );

export const getCurrentUser = async (): Promise<User | undefined> => {
    const token = getToken();

    if (!token) return undefined;

    return await (
        await fetch("/api/v1/users/me", {
            headers: {
                Authorization: `Bearer ${token}`,
            },
        })
    ).json();
};
