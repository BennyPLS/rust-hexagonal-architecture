-- Your SQL goes here
CREATE TABLE users
(
    id       UUID NOT NULL PRIMARY KEY,
    name     TEXT NOT NULL,
    password TEXT NOT NULL,
    email    TEXT NOT NULL
);

