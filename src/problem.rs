use std::error::Error;

use axum::{response::{IntoResponse, Response}, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

/// An issue is a single form error message.
type Issue = Vec<String>;

/// The error returned when a form is invalid.
#[derive(Serialize, Deserialize, Debug)]
pub struct FormErr {
	pub field: String,
	pub issues: Issue,
}

/// The problem that can be returned when an endpoint errors.
#[derive(Serialize, Deserialize, Debug)]
pub struct Problem {
    #[serde(skip_serializing, skip_deserializing)]
	pub status: StatusCode,
	pub title: String,
	pub detail: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub form: Option<Vec<FormErr>>,
}

impl Problem {
	pub fn from_form(value: Vec<FormErr>) -> Self {
		Self {
			status: StatusCode::BAD_REQUEST,
			title: "Invalid form".to_string(),
			detail: "Invalid form data was recived.".to_string(),
			form: Some(value) 
		}
	}
}

impl IntoResponse for Problem {
	fn into_response(self) -> Response {
		(self.status, Json(self)).into_response()
	}
}

impl<E> From<E> for Problem
where
	E: Error,
{
	fn from(value: E) -> Self {
		Self {
			status: StatusCode::INTERNAL_SERVER_ERROR,
			title: "Internal server error".to_string(),
			detail: value.to_string(),
			form: None
		}
	}
}
