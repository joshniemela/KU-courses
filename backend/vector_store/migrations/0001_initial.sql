-- Initial database migration for the vector search
BEGIN;

CREATE TABLE IF NOT EXISTS course (
    id text PRIMARY KEY,
    title text NOT NULL,
    content text NOT NULL,
    last_modified timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS coordinator (
       email text PRIMARY KEY,
       full_name text NOT NULL
);

CREATE TABLE IF NOT EXISTS course_coordinator (
    id text,
    email text,
    PRIMARY KEY (id, email),
    FOREIGN KEY (id) REFERENCES course(id),
    FOREIGN KEY (email) REFERENCES coordinator(email)
);

CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE IF NOT EXISTS name_embedding (
    email text PRIMARY KEY,
    embedding vector(384) NOT NULL,
    FOREIGN KEY (email) REFERENCES coordinator(email)
);

CREATE TABLE IF NOT EXISTS title_embedding (
    course_id text PRIMARY KEY,
    embedding vector(384) NOT NULL,
    last_modified timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (course_id) REFERENCES course(id)
);

CREATE TABLE IF NOT EXISTS content_embedding (
    id serial PRIMARY KEY,
    course_id text NOT NULL,
    embedding vector(384) NOT NULL,
    last_modified timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (course_id) REFERENCES course(id)
);

COMMIT;
