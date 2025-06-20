CREATE TABLE link (
    id serial PRIMARY KEY,
    name varchar(255),
    description text,
    content text NOT NULL,
    created_at timestamp DEFAULT now() NOT NULL
);