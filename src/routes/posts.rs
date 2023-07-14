use axum::{routing::post, Router};

pub struct Posts;

impl Posts {
	/// Returns the routes for the auth module.
	pub fn routes() -> Router<sqlx::PgPool> {
		Router::new()
			.route("/create", post(Self::create))
			.route("/delete", post(Self::delete))
	}

	/// Create a post
	pub async fn create() {
		unimplemented!()
	}

	/// Delete a post
	pub async fn delete() {
		unimplemented!()
	}
}