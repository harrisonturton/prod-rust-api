import schema from "./auth_schema.json";
import { HttpService, ApiClient } from "services/http";
import {
    SignInRequest,
    SignInResponse,
    SignOutRequest,
    SignOutResponse,
} from "services/auth/auth_schema";

export interface AuthService {
    signIn(req: SignInRequest): Promise<SignInResponse>;
    signOut(req: SignOutRequest): Promise<SignOutResponse>;
}

export class AuthClient implements AuthService {
    private static BASE: string = "auth";

    private readonly client: ApiClient;

    constructor(httpService: HttpService) {
        this.client = new ApiClient(AuthClient.BASE, httpService);
    }

    signIn(req: SignInRequest): Promise<SignInResponse> {
        return this.client.post(req, {
            path: "/sign_in",
            request: schema.definitions.SignInRequest,
            response: schema.definitions.SignInResponse,
        });
    }

    signOut(req: SignOutRequest): Promise<SignOutResponse> {
        return this.client.post(req, {
            path: "/sign_out",
            request: schema.definitions.SignOutRequest,
            response: schema.definitions.SignOutResponse,
        });
    }
}
