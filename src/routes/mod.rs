use axum::Router;

use crate::AppState;

pub mod auth;

pub struct Routes;

impl Routes {
	pub fn generate() -> Router<AppState> {
		Router::new().nest("/auth", auth::Auth::routes())
	}
}
