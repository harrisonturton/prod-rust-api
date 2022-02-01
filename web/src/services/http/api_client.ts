import { HttpService } from "services/http";
import { checkSchema, Schema } from "services/schema/schema";

export interface GetRoute {
    path: string;
    response: Schema;
}

export interface PostRoute {
    path: string;
    request: Schema;
    response: Schema;
}

/**
 * Since most API methods follow the same structure, this abstracts over that
 * logic to keep each service client implementation consistent. Eventually we'll
 * want to swap these hand-written implementations with a fully generated RPC
 * solution (e.g. gRPC), but this is easy enough for now.
 */
export class ApiClient {
    private readonly base: string;
    private readonly httpService: HttpService;

    constructor(base: string, httpService: HttpService) {
        this.base = base;
        this.httpService = httpService;
    }

    /**
     * Make a GET request. Validates the response body and throws on failure.
     *
     * @param route the route to send the request to.
     * @returns the response body on success.
     */
    async get<Res extends object>(route: GetRoute): Promise<Res> {
        let { path, response } = route;
        let url = `${this.base}${path}`;
        let res = await this.httpService.get(url);
        checkSchema(res, response, `bad request schema on GET ${url}`);
        return res as Res;
    }

    /**
     * Make a POST request. Validates the request and response body. Throws on
     * failure.
     *
     * @param req the request body to post.
     * @param route the route to send the request to.
     * @returns the response body on success.
     */
    async post<Req extends object, Res extends object>(
        req: Req,
        route: PostRoute
    ): Promise<Res> {
        let { path, request, response } = route;
        let url = `${this.base}${path}`;
        checkSchema(req, request, `bad request schema on POST ${url}`);
        let res = await this.httpService.post(url, req);
        checkSchema(res, response, `bad response schema on POST ${url}`);
        return res as Res;
    }
}
