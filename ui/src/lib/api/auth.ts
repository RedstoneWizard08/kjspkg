import Cookies from "js-cookie";

export const getToken = () => Cookies.get("auth-token");
export const isLoggedIn = () => !!getToken();
