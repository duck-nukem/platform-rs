CREATE TABLE users (
    id              BIGSERIAL PRIMARY KEY,
    name            VARCHAR(25) NOT NULL,
    password_hash   VARCHAR(60) NOT NULL
);