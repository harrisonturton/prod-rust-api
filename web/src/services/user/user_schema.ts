export interface GetUserRequest {
    readonly by_id: string
}

export interface GetUserResponse {
    readonly user: User
}

export interface User {
    readonly id: string,
    readonly email: string,
    readonly created_at: Date,
}