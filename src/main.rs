mod database;
mod model;
mod problem;
mod routes;

use std::net::{Ipv4Addr, SocketAddrV4};

use routes::Routes;

#[tokio::main]
async fn main() {
	tracing_subscriber::registry();

	if let Err(err) = dotenv::dotenv() {
		panic!("Failed to load .env file: {}", err);
	};
	let Ok(conn_str) = std::env::var("DATABASE_URL") else {
		panic!("DATABASE_URL environment variable not set");
	};
	let Ok(pool) = sqlx::PgPool::connect(&conn_str).await else {
		panic!("Failed to connect to database: {}", conn_str);
	};

	sqlx::migrate!().run(&pool).await.unwrap();

	let app = axum::Router::new()
		.nest("/api", Routes::generate())
		.with_state(pool);

	let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8080).into();
	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}