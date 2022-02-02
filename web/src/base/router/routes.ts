export type RoutePath =
    | "/"
    | "/editor"
    | "/sign_in"
    | "/sign_out"
    | "/settings";

export type QueryParams = { [key: string]: string };

export interface Route {
    path: RoutePath;
    query?: QueryParams;
}

export const getEditorRoute = (query?: QueryParams): Route => ({
    path: "/editor",
    query,
});
