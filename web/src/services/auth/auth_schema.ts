export interface SignInRequest {
    readonly email: string,
    readonly password: string,
}

export interface SignInResponse {
    readonly token: string,
};

export interface SignOutRequest {
    readonly token: string,
} 

export interface SignOutResponse {} 