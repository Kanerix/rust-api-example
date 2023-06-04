use serde::{Deserialize, Serialize};

type Issue = Vec<String>;

/// The errors that can be returned when an endpoint errors.
#[derive(Serialize, Deserialize, Debug)]
pub enum Err {
	#[serde(rename = "error")]
	StdErr(StdErr),
	#[serde(rename = "form")]
	FormErr(Vec<FormErr>),
}

/// The normal error returned when an endpoint errors.
#[derive(Serialize, Deserialize, Debug)]
pub struct StdErr {
	pub error: String,
}

/// The error returned when a form is invalid.
#[derive(Serialize, Deserialize, Debug)]
pub struct FormErr {
	pub field: String,
	pub issues: Issue,
}
