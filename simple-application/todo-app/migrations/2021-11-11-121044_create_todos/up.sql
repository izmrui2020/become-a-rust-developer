-- Your SQL goes here
CREATE TABLE todos (
  id        VARCHAR NOT NULL PRIMARY KEY,
  kind      VARCHAR NOT NULL,
  contents  VARCHAR NOT NULL,
  done      BOOLEAN NOT NULL DEFAULT 0
)