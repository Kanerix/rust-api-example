-- Add migration script here
CREATE TABLE users (
  id 			SERIAL PRIMARY KEY,
  username 		VARCHAR(32) NOT NULL UNIQUE,
  email 		VARCHAR(64) NOT NULL UNIQUE,
  password 		VARCHAR(100) NOT NULL,
  created_at 	TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at 	TIMESTAMPTZ NOT NULL DEFAULT NOW()
);