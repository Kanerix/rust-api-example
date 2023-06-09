CREATE TABLE refresh_tokens (
	id			SERIAL PRIMARY KEY,
	token 		VARCHAR(255) NOT NULL,
	user_id 	INTEGER NOT NULL,
	created_at 	TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at 	TIMESTAMP NOT NULL DEFAULT NOW(),
	FOREIGN KEY (user_id) REFERENCES users(id)
);