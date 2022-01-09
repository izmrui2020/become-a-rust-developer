-- Your SQL goes here
CREATE TABLE todos (
    id          SERIAL PRIMARY KEY,
    kind        VARCHAR NOT NULL,
    contents    VARCHAR NOT NULL,
    done        BOOLEAN NOT NULL DEFAULT false
);

-- INSERT INTO todos (kind, contents, done) VALUES ("private", "go to shop", false);
-- INSERT INTO todos (kind, contents, done) VALUES ("work", "write code by rust", false);
-- INSERT INTO todos (kind, contents, done) VALUES ("private", "hogehoge", true);