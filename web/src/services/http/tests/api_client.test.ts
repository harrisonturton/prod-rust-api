import { ApiClient } from "../api_client";
import { HttpService } from "../http_client";

class FakeHttpService implements HttpService {
    async get(path: string): Promise<object> {
        return {};
    }
    async post(path: String, body?: object): Promise<object> {
        return {};
    }
}

describe("ApiClient", () => {
    it("works", () => {
        expect(1).toBe(1);
    });
});
