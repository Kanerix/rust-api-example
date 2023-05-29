use axum::Router;

pub mod auth;

pub struct Routes;

impl Routes {
	pub fn new() -> Router {
		Router::new().nest("/auth", auth::Auth::new())
	}
}
