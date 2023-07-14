# rust-api-example

This is an example repo that shows how i would implement a REST API using the Rust programming language.

!! To run this project you must have Rust installed on your system !!

### Requirements

#### Generate Public and Private key for JWT.

Run these commands:

1. `openssl genpkey -algorithm ed25519 -out keys/private.pem`
2. `openssl pkey -in keys/private.pem -pubout -out keys/public.pem`

### Start the project

- Run the command `cargo run`

### Using docker

- Run `docker compose up -d` to start the project running in the background