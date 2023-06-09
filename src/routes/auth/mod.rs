pub mod guard;
pub mod hashing;
pub mod jwt;

use axum::{
	extract::State,
	headers::{Authorization, HeaderMapExt},
	http::{HeaderMap, StatusCode},
	response::IntoResponse,
	routing::post,
	Form, Router,
};
use serde::{Deserialize, Serialize};

use crate::{
	model::User,
	problem::Problem,
	routes::auth::{
		hashing::{hash_password, verify_password},
		jwt::{generate_access_token, generate_refresh_token},
	},
};

/// Input for the `register` route.
#[derive(Deserialize, Debug)]
pub struct RegisterPayload {
	email: String,
	username: String,
	password: String,
}

/// Input for the `login` route.
#[derive(Deserialize, Debug)]
pub struct LoginPayload {
	email: String,
	password: String,
}

/// Returned as the response for a successful login.
#[derive(Serialize, Debug)]
pub struct AuthResponse {
	refresh_token: String,
	access_token: String,
}

pub struct Auth;

impl Auth {
	/// Returns the routes for the auth module.
	pub fn routes() -> Router<sqlx::PgPool> {
		Router::new()
			.route("/register", post(Self::register))
			.route("/login", post(Self::login))
	}

	/// The handler for registering a new user.
	pub async fn register(
		State(pool): State<sqlx::PgPool>,
		Form(payload): Form<RegisterPayload>,
	) -> Result<impl IntoResponse, Problem> {
		let password_hash = hash_password(payload.password.as_bytes())?;
		let user = sqlx::query!(
			"INSERT INTO users (email, username, password) VALUES ($1, $2, $3)",
			&payload.email,
			&payload.username,
			&password_hash
		)
		.execute(&pool)
		.await;

		if let Err(err) = user {
			if let sqlx::Error::Database(db_err) = err {
				if let Some(constraint) = db_err.constraint() {
					let formatted = constraint.replace("users_", "").replace("_key", "");
					return Err(Problem {
						status: StatusCode::BAD_REQUEST,
						title: format!("Invalid {formatted}"),
						detail: format!("The {formatted} already exsists."),
					});
				}
			}

			return Err(Problem {
				status: StatusCode::BAD_REQUEST,
				title: "Failed to register".to_string(),
				detail: "Failed to register user.".to_string(),
			});
		}

		Ok(StatusCode::CREATED)
	}

	/// The handler for logging in.
	pub async fn login(
		State(pool): State<sqlx::PgPool>,
		Form(payload): Form<LoginPayload>,
	) -> Result<impl IntoResponse, Problem> {
		let user = sqlx::query_as_unchecked!(
			User,
			"SELECT * FROM users WHERE email = $1",
			&payload.email,
		)
		.fetch_one(&pool)
		.await;

		if let Err(sqlx::Error::RowNotFound) = user {
			return Err(Problem {
				status: StatusCode::BAD_REQUEST,
				title: "Invalid email or password".to_string(),
				detail: "The email or password used is invalid.".to_string(),
			});
		}

		let user = user?;

		if verify_password(&user.password, &payload.password.as_bytes())? {
			return Err(Problem {
				status: StatusCode::BAD_REQUEST,
				title: "Invalid email or password".to_string(),
				detail: "The email or password used is invalid.".to_string(),
			});
		}

		let refresh_token = generate_refresh_token();
		let access_token = generate_access_token(&user)?;
		sqlx::query!(
			"INSERT INTO refresh_tokens (user_id, token) VALUES ($1, $2)",
			user.id,
			&refresh_token
		)
		.execute(&pool)
		.await?;

		let auth_header = Authorization::bearer(&access_token)?;
		let mut headers = HeaderMap::new();
		headers.typed_insert(auth_header);

		Ok(headers)
	}
}
