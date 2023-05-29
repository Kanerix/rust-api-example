use axum::{http::StatusCode, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

use crate::util::validate_password;

pub struct Auth;

/// Input for the `register` route.
#[derive(Deserialize, Debug)]
pub struct CreateUser {
	username: String,
	password: String,
	email: String,
}

impl Auth {
	pub fn new() -> Router {
		Router::new()
			.route("/register", post(Self::register))
			.route("/login", post(Self::login))
			.route("/logout", post(Self::logout))
	}

	pub async fn register(
		Json(payload): Json<CreateUser>
	) -> (StatusCode, Result<(), String>) {
		if let Err(err) = validate_password(payload.password.as_str()) {
			return (StatusCode::BAD_REQUEST, Err(err.to_string()))
		};
		(StatusCode::OK, Ok(()))
	}

	pub async fn login() -> &'static str {
		""
	}

	pub async fn logout() -> &'static str {
		""
	}
}
