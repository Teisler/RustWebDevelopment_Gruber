-- Add up migration script here
CREATE TABLE IF NOT EXISTS answers (
    id serial PRIMARY KEY,
    content text NOT NULL,
    created_on TIMESTAMP NOT NULL DEFAULT NOW(),
    corresponding_question integer REFERENCES questions
);