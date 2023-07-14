use axum::Router;

pub mod auth;
pub mod posts;

pub struct Routes;

impl Routes {
	pub fn generate() -> Router<sqlx::PgPool> {
		Router::new()
			.nest("/auth", auth::Auth::routes())
			.nest("/posts", posts::Posts::routes())
	}
}
