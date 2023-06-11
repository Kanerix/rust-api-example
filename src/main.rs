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
pub struct AppEnv {
	jwt_key: String,
	hash_salt: String,
}

#[derive(Clone)]
pub struct AppState {
	db_pool: sqlx::PgPool,
	env: AppEnv,
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

	let jwt_key = env::var("SECRET_JWT_KEY").unwrap();
	let hash_salt = env::var("SECRET_HASH_SALT").unwrap();

	let env = AppEnv { jwt_key, hash_salt };
	let shared_state = AppState {
		db_pool,
		env,
	};
	let app = Router::new()
		.nest("/api", Routes::generate())
		.with_state(shared_state);

	let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8080).into();
	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}