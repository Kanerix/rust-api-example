mod utils;

use axum::{
	extract::State,
	http::{header::SET_COOKIE, StatusCode, HeaderMap},
	response::IntoResponse,
	routing::post,
	Router, Form,
};
use axum_extra::extract::cookie::Cookie;
use serde::{Deserialize, Serialize};

use sqlx::Error;

use crate::{
	model::User,
	problem::Problem,
	AppState, routes::auth::utils::hash_password,
};

use self::utils::{generate_access_token, generate_refresh_token};

pub struct Auth;

/// Returned as the payload for a successful login.
#[derive(Serialize, Debug)]
pub struct AuthPayload {
	refresh_token: String,
	access_token: String,
}

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

impl Auth {
	/// Returns the routes for the auth module.
	pub fn routes() -> Router<AppState> {
		Router::new()
			.route("/register", post(Self::register))
			.route("/login", post(Self::login))
	}

	/// The handler for registering a new user.
	pub async fn register(
		State(state): State<AppState>,
		Form(payload): Form<RegisterPayload>,
	) -> Result<impl IntoResponse, Problem> {
		let password_hash = hash_password(payload.password.as_bytes())?;
		dbg!(&password_hash);
		dbg!(&payload.email);
		let user = sqlx::query!(
			"INSERT INTO users (email, username, password) VALUES ($1, $2, $3)",
			&payload.email,
			&payload.username,
			&password_hash
		)
		.execute(&state.db_pool)
		.await;

		if let Err(err) = user {
			if let Error::Database(db_err) = err {
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
		State(state): State<AppState>,
		Form(payload): Form<LoginPayload>,
	) -> Result<impl IntoResponse, Problem> {
		let password_hash = hash_password(&payload.password.as_bytes())?;
		dbg!(&password_hash);
		dbg!(&payload.email);
		let user = sqlx::query_as_unchecked!(
			User,	
			"SELECT * FROM users WHERE email = $1 AND password = $2",
			&payload.email,
			&password_hash
		)
		.fetch_one(&state.db_pool)
		.await;

		dbg!(&user);
		if let Err(Error::RowNotFound) = user {
			return Err(Problem {
				status: StatusCode::BAD_REQUEST,
				title: "Invalid email or password".to_string(),
				detail: "The email or password used is invalid.".to_string(),
			})
		}

		let user = user?;

		if !argon2::verify_encoded(&user.password, payload.password.as_bytes())? {
			return Err(Problem {
				status: StatusCode::BAD_REQUEST,
				title: "Invalid email or password".to_string(),
				detail: "The email or password used is invalid.".to_string(),
			});
		}

		let refresh_token = generate_refresh_token(32);
		let access_token = generate_access_token(&user)?;
		sqlx::query!(
			"INSERT INTO refresh_tokens (user_id, token) VALUES ($1, $2)",
			user.id,
			&refresh_token
		)
		.execute(&state.db_pool)
		.await?;

		let refresh_token_cookie = Cookie::build("refresh_token", refresh_token)
			.path("/")
			.finish()
			.to_string();
		let access_token_cookie = Cookie::build("access_token", access_token)
			.path("/")
			.finish()
			.to_string();

		let mut headers = HeaderMap::new();
		headers.append(SET_COOKIE, refresh_token_cookie.parse()?);
		headers.append(SET_COOKIE, access_token_cookie.parse()?);

		Ok(headers)
	}
}
