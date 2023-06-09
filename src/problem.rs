use axum::{
	http::StatusCode,
	response::{IntoResponse, Response},
	Json,
};
use serde::{Deserialize, Serialize};

/// The problem that can be returned when an endpoint errors.
#[derive(Serialize, Deserialize, Debug)]
pub struct Problem {
	#[serde(skip_serializing, skip_deserializing)]
	pub status: StatusCode,
	pub title: String,
	pub detail: String,
}

/// How the problem is turned into a response.
impl IntoResponse for Problem {
	fn into_response(self) -> Response {
		(self.status, Json(self)).into_response()
	}
}

/// Allows you to return any error as an internal server error.
impl<E> From<E> for Problem
where
	E: Into<anyhow::Error>,
{
	fn from(value: E) -> Self {
		Self {
			status: StatusCode::INTERNAL_SERVER_ERROR,
			title: "Internal server error".to_string(),
			detail: value.into().to_string(),
		}
	}
}
