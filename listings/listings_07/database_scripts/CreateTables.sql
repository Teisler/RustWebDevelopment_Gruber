CREATE TABLE IF NOT EXISTS questions (
    id serial PRIMARY KEY,
    title VARCHAR (255) NOT NULL,
    content TEXT NOT NULL,
    tags TEXT [],
    created_on TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS answers (
  id serial PRIMARY KEY,
  content TEXT NOT NULL,
  created_on TIMESTAMP NOT NULL DEFAULT now(),
  corresponding_question integer REFERENCES questions
);