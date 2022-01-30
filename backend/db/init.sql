DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS session;

CREATE TABLE users (
    private_id SERIAL PRIMARY KEY,
    id CHAR(11) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    hash CHAR(88) NOT NULL
);

CREATE TABLE session (
    private_id SERIAL PRIMARY KEY,
    user_id CHAR(11) NOT NULL,
    token CHAR(256) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);