mod routes;
mod util;

use core::panic;
use std::{
	net::{Ipv4Addr, SocketAddrV4},
	sync::Arc,
};

use axum::Router;

struct AppState {
	db_pool: sqlx::PgPool,
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

	let shared_state = Arc::new(AppState { db_pool });

	let app = Router::new()
		.with_state(shared_state)
		.nest("/api", routes::Routes::new());
	let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8080).into();
	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}
