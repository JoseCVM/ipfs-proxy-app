-- Your SQL goes here
CREATE TABLE requests (
  id SERIAL NOT NULL PRIMARY KEY,
  api_key TEXT NOT NULL,
  api_call TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL
);