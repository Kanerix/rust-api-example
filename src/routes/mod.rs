use axum::Router;

use crate::AppState;

pub mod auth;

pub struct Routes;

impl Routes {
	pub fn new() -> Router<AppState> {
		Router::new().nest("/auth", auth::Auth::new())
	}
}
