mod utils;

use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use serde::Deserialize;

use sqlx::Error;
use utils::{validate_email, validate_password, validate_username};

use crate::{
	error::{Err, FormErr, StdErr},
	AppState,
};

pub struct Auth;

/// Input for the `register` route.
#[derive(Deserialize, Debug)]
pub struct CreateUser {
	email: String,
	username: String,
	password: String,
}

impl Auth {
	pub fn new() -> Router<AppState> {
		Router::new()
			.route("/register", post(Self::register))
			.route("/login", post(Self::login))
			.route("/logout", post(Self::logout))
	}

	pub async fn register(
		State(state): State<AppState>,
		Json(payload): Json<CreateUser>,
	) -> (StatusCode, Result<(), Json<Err>>) {
		let mut form_errors: Vec<FormErr> = Vec::new();

		if let Err(err) = validate_email(&payload.email) {
			form_errors.push(FormErr {
				field: "email".to_string(),
				issues: err.into_iter().map(|e| e.to_string()).collect(),
			})
		};

		if let Err(err) = validate_username(&payload.username) {
			form_errors.push(FormErr {
				field: "username".to_string(),
				issues: err.into_iter().map(|e| e.to_string()).collect(),
			})
		};

		if let Err(err) = validate_password(&payload.password) {
			form_errors.push(FormErr {
				field: "password".to_string(),
				issues: err.into_iter().map(|e| e.to_string()).collect(),
			})
		};

		if !form_errors.is_empty() {
			return (StatusCode::BAD_REQUEST, Err(Json(Err::FormErr(form_errors))));
		}

		let user = sqlx::query!(
			"INSERT INTO users (email, username, password) VALUES ($1, $2, $3)",
			&payload.email,
			&payload.username,
			&payload.password
		)
		.execute(&state.db_pool)
		.await;

		if let Err(err) = user {
			if let Error::Database(db_err) = err {
				if let Some(constraint) = db_err.constraint() {
					let constraint_string = constraint.replace("users_", "").replace("_key", "");
					return (
						StatusCode::BAD_REQUEST,
						Err(Json(Err::StdErr(StdErr {
							error: format!("The {} is taken", constraint_string),
						}))),
					);
				}	
			}

			return (StatusCode::INTERNAL_SERVER_ERROR, Err(Json(
				Err::StdErr(StdErr {
					error: "Failed to create user".to_string(),
				})
			)));
		}

		(StatusCode::OK, Ok(()))
	}

	pub async fn login() -> &'static str {
		""
	}

	pub async fn logout() -> &'static str {
		""
	}
}
