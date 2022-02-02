import schema from "./user_schema.json";
import { HttpService, ApiClient } from "services/http";
import {
    ListUsersResponse,
} from "services/user/user_schema";

export interface UserService {
    listUsers(): Promise<ListUsersResponse>;
}

export class UserClient implements UserService {
    private static SERVICE_SLUG: "user" = "user";

    private readonly client: ApiClient;

    constructor(httpService: HttpService) {
        this.client = new ApiClient(UserClient.SERVICE_SLUG, httpService);
    }

    async listUsers(): Promise<ListUsersResponse> {
        return this.client.get({
            path: "/",
            response: schema.definitions.ListUsersResponse,
        });
    }
}