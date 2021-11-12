-- Your SQL goes here
CREATE TABLE todos (
    id          INTEGER NOT NULL PRIMARY KEY,
    kind        VARCHAR NOT NULL PRIMARY KEY,
    contents    VARCHAR NOT NULL PRIMARY KEY,
    done        BOOLEAN NOT NULL DEFAULT 0
);

INSERT INTO todos(kind, contents, done) VALUES("private", "go to shop", false);
INSERT INTO todos(kind, contents, done) VALUES("work", "write code by rust", false);
INSERT INTO todos(kind, contents, done) VALUES("private", "hogehoge", true);