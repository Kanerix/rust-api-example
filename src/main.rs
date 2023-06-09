#![feature(try_trait_v2)]
mod model;
mod problem;
mod routes;

use std::{
	env,
	net::{Ipv4Addr, SocketAddrV4},
};

use axum::Router;
use routes::Routes;

#[derive(Clone)]
pub struct AppState {
	db_pool: sqlx::PgPool,
	secret_jwt_key: String,
	secret_salt_key: String,
}

#[tokio::main]
async fn main() {
	if let Err(err) = dotenv::dotenv() {
		panic!("Failed to load .env file: {}", err);
	};
	let Ok(conn_str) = std::env::var("DATABASE_URL") else {
		panic!("DATABASE_URL environment variable not set");
	};
	let Ok(db_pool) = sqlx::PgPool::connect(&conn_str).await else {
		panic!("Failed to connect to database: {}", conn_str);
	};

	let secret_jwt_key = env::var("SECRET_JWT_KEY").unwrap();
	let secret_salt_key = env::var("SECRET_HASH_SALT").unwrap();

	let shared_state = AppState {
		db_pool,
		secret_jwt_key,
		secret_salt_key,
	};
	let app = Router::new()
		.nest("/api", Routes::new())
		.with_state(shared_state);

	let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8080).into();
	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}
