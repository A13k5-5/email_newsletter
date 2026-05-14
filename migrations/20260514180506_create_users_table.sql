-- Add migration script here
CREATE TABLE users(
    user_id uuid NOT NULL,
    PRIMARY KEY (user_id),
    username TEXT NOT NULL,
    password TEXT NOT NULL
);