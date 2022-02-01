/**
 * This interface is used so we can mock the `HttpClient` for unit tests.
 */
export interface HttpService {
    get(path: string): Promise<object>;
    post(path: String, body?: object): Promise<object>;
}

export class HttpClient implements HttpService {
    private readonly baseUrl: string;

    constructor(baseUrl: string) {
        this.baseUrl = baseUrl;
    }

    /**
     * Make a GET request. Does not validate the response.
     *
     * @param path path part to send the request to.
     * @returns JSON body of the response.
     */
    async get(path: string): Promise<object> {
        let res = await fetch(this.baseUrl + path, {
            method: "GET",
        });
        return res.json();
    }

    /**
     * Make a POST request. Does not validate the request or response.
     *
     * @param path path part to send the request to.
     * @param body request body to serialize.
     * @returns JSON body of the response.
     */
    async post(path: string, body?: object): Promise<object> {
        console.log("POSTING", `${this.baseUrl}/${path}`, body);
        let res = await fetch(`${this.baseUrl}/${path}`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(body),
        });
        return res.json();
    }
}
