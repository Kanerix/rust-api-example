mod utils;

use argon2::Config;
use axum::{
	extract::State,
	http::{header::SET_COOKIE, Response, StatusCode},
	response::IntoResponse,
	routing::post,
	Router, Form,
};
use axum_extra::extract::cookie::Cookie;
use serde::{Deserialize, Serialize};

use sqlx::Error;
use utils::{validate_email, validate_password, validate_username};

use crate::{
	model::User,
	problem::{FormErr, Problem},
	AppState,
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
	pub fn new() -> Router<AppState> {
		Router::new()
			.route("/register", post(Self::register))
			.route("/login", post(Self::login))
	}

	/// The handler for registering a new user.
	pub async fn register(
		State(state): State<AppState>,
		Form(payload): Form<RegisterPayload>,
	) -> Result<impl IntoResponse, Problem> {
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
			return Err(Problem::from_form(form_errors));
		}

		let hash = argon2::hash_encoded(
			&payload.password.as_bytes(),
			&state.env.hash_salt.as_bytes(),
			&Config::default(),
		)
		.unwrap();

		let user = sqlx::query!(
			"INSERT INTO users (email, username, password) VALUES ($1, $2, $3)",
			&payload.email,
			&payload.username,
			&hash
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
						form: None
					});
				}
			}

			return Err(Problem {
				status: StatusCode::BAD_REQUEST,
				title: "Failed to register".to_string(),
				detail: "Failed to register user.".to_string(),
				form: None
			});
		}

		Ok(())
	}

	/// The handler for logging in.
	pub async fn login(
		State(state): State<AppState>,
		Form(payload): Form<LoginPayload>,
	) -> Result<impl IntoResponse, Problem> {
		let hash = argon2::hash_encoded(
			&payload.password.as_bytes(),
			&state.env.hash_salt.as_bytes(),
			&Config::default(),
		)?;

		let user = match sqlx::query_as!(
			User,
			"SELECT * FROM users WHERE email = $1 AND password = $2",
			&payload.email,
			&hash,
		)
		.fetch_one(&state.db_pool)
		.await
		{
			Ok(user) => user,
			Err(err) => {
				if let Error::RowNotFound = err {
					return Err(Problem {
						status: StatusCode::BAD_REQUEST,
						title: "Invalid email or password".to_string(),
						detail: "The email or password used is invalid.".to_string(),
						form: None
					})
				}

				return Err(Problem {
					status: StatusCode::BAD_REQUEST,
					title: "Failed to login".to_string(),
					detail: "Failed to login user.".to_string(),
					form: None
				})
			}
		};

		let matches = argon2::verify_encoded(&user.password, &payload.password.as_bytes())?;

		if !matches {
			return Err(Problem {
				status: StatusCode::BAD_REQUEST,
				title: "Invalid email or password".to_string(),
				detail: "The email or password used is invalid.".to_string(),
				form: None
			});
		}

		let refresh_token = generate_refresh_token(32);
		let access_token = generate_access_token(&user, &state.env.jwt_key)?;
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
		let builder = Response::builder()
			.header(SET_COOKIE, refresh_token_cookie)
			.header(SET_COOKIE, access_token_cookie);

		Ok(builder.body("".to_string())?)
	}
}
