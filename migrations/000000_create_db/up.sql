-- Your SQL goes here

CREATE TABLE users (
  id SERIAL NOT NULL PRIMARY KEY,
  username TEXT NOT NULL UNIQUE,
  created_at TIMESTAMP NOT NULL
);

CREATE TABLE keys (
  api_key TEXT NOT NULL PRIMARY KEY,
  expires_in INT NOT NULL,
  is_enabled BOOLEAN NOT NULL,
  userid INT NOT NULL REFERENCES users (id),
  created_at TIMESTAMP NOT NULL
);

CREATE TABLE requests (
  id SERIAL NOT NULL PRIMARY KEY,
  api_key TEXT NOT NULL REFERENCES keys (api_key),
  api_call TEXT NOT NULL,
  request_size INT NOT NULL,
  created_at TIMESTAMP NOT NULL
);