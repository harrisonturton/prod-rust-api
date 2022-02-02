export interface ListUsersResponse {
    readonly users: User[];
}

export type FindUserRequest = FindUserByIdRequest | FindUserByEmailRequest;

export interface FindUserByIdRequest {
    readonly by_id: string;
}

export interface FindUserByEmailRequest {
    readonly by_email: string;
}

export interface FindUserResponse {
    readonly user: User;
}

export interface CreateUserRequest {
    readonly email: string;
    readonly password: string;
}

export interface CreateUserResponse {
    readonly user: User;
}

export interface User {
    readonly id: string;
    readonly email: string;
    readonly hash: string;
    readonly created_at: string;
}
