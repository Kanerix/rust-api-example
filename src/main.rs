mod model;
mod problem;
mod routes;

use std::net::{Ipv4Addr, SocketAddrV4};

use routes::Routes;

#[derive(Clone)]
pub struct AppState {
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

    sqlx::migrate!().run(&db_pool).await.unwrap();

	let shared_state = AppState {
		db_pool,
	};
	let app = axum::Router::new()
		.nest("/api", Routes::generate())
		.with_state(shared_state);

	let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8080).into();
	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}