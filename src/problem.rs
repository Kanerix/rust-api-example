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

/// The diffrent variants of problem that can occurre.
#[derive(Serialize, Deserialize, Debug)]
pub enum ProblemVariant {
	#[serde(rename = "standard")]
	StdErr,
	#[serde(rename = "form")]
	FormErr(Vec<FormErr>),
}

/// The problem that can be returned when an endpoint errors.
#[derive(Serialize, Deserialize, Debug)]
pub struct Problem {
    #[serde(skip_serializing, skip_deserializing)]
	pub status: StatusCode,
	pub short: String,
	pub message: String,
	pub variant: ProblemVariant,
}

impl Problem {
	pub fn from_form(value: Vec<FormErr>) -> Self {
		Self {
			status: StatusCode::BAD_REQUEST,
			short: "Invalid form".to_string(),
			message: "Invalid form data was recived.".to_string(),
			variant: ProblemVariant::FormErr(value),
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
			short: "Internal server error".to_string(),
			message: value.to_string(),
			variant: ProblemVariant::StdErr,
		}
	}
}
