import { useRouter as useNextRouter, NextRouter } from "next/router";
import { Route } from "./routes";

export interface RouterController {
    pushRoute(route: Route): void;
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
        let query = new URLSearchParams(route.query).toString();
        let url = `${route.path}?${query}`;
        this.nextRouter.push(url);
    }
}
