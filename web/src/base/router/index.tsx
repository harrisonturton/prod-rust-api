import { useRouter as useNextRouter, NextRouter } from "next/router";

export type RoutePath = "/" | "/editor";

export type QueryParams = { [key: string]: string };

export interface Route {
    path: RoutePath,
    query?: QueryParams,
}

export const getEditorRoute = (query?: QueryParams): Route => ({
    path: "/editor",
    query,
})

export interface RouterController {
    pushRoute(route: Route): void,
}

/**
 * A hook for handle page routing from within a stateful component.  
 * 
 * @returns RouterController instance.
 */
export class Router implements RouterController {
    private readonly nextRouter: NextRouter;

    constructor() {
        this.nextRouter = useNextRouter();
    }

    pushRoute(route: Route) {
        this.nextRouter.push(route.path);
    }
}