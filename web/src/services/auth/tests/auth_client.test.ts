import { AuthClient } from "services/auth";
import { HttpClient, HttpService } from "services/http";

describe("AuthClient", () => {
    let httpService: HttpService;
    let client: AuthClient;

    beforeEach(() => {
        httpService = new HttpClient("baseUrl");
        client = new AuthClient(httpService);
    });

    it("throws on invalid signIn requests", async () => {
        expect(
            client.signIn({ param: "Bad request" } as any)
        ).rejects.toThrow();
    });

    it("throws on invalid signOut requests", async () => {
        expect(
            client.signOut({ param: "Bad request" } as any)
        ).rejects.toThrow();
    });
});
