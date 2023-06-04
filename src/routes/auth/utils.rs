use std::fmt;

use regex::Regex;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum EmailError {
	TooShort,
	TooLong,
	WrongFormat,
}

#[derive(Serialize, Debug)]
pub enum UsernameError {
	TooShort,
	TooLong,
	SpecialCharacters,
}

#[derive(Serialize, Debug)]
pub enum PasswordError {
	TooShort,
	TooLong,
	NoSpecialCharacters,
	NoUppercase,
	NoLowercase,
	NoNumbers,
}

impl fmt::Display for EmailError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			EmailError::TooShort => write!(f, "Too short"),
			EmailError::TooLong => write!(f, "Too long"),
			EmailError::WrongFormat => write!(f, "Wrong format"),
		}
	}
}

impl fmt::Display for UsernameError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			UsernameError::TooShort => write!(f, "Too short"),
			UsernameError::TooLong => write!(f, "Too long"),
			UsernameError::SpecialCharacters => write!(f, "Special characters"),
		}
	}
}

impl fmt::Display for PasswordError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			PasswordError::TooShort => write!(f, "Too short"),
			PasswordError::TooLong => write!(f, "Too long"),
			PasswordError::NoSpecialCharacters => write!(f, "No special characters"),
			PasswordError::NoUppercase => write!(f, "No uppercase"),
			PasswordError::NoLowercase => write!(f, "No lowercase"),
			PasswordError::NoNumbers => write!(f, "No numbers"),
		}
	}
}

pub fn validate_password<'a>(password: &'a str) -> Result<(), Vec<PasswordError>> {
	let mut issues = Vec::new();

	if password.len() < 8 {
		issues.push(PasswordError::TooShort);
	}
	if password.len() > 100 {
		issues.push(PasswordError::TooLong);
	}
	if !password.chars().any(|c| c.is_ascii_uppercase()) {
		issues.push(PasswordError::NoUppercase);
	}
	if !password.chars().any(|c| c.is_ascii_lowercase()) {
		issues.push(PasswordError::NoLowercase);
	}
	if !password.chars().any(|c| c.is_ascii_digit()) {
		issues.push(PasswordError::NoNumbers);
	}
	if !password.chars().any(|c| !c.is_ascii_alphanumeric()) {
		issues.push(PasswordError::NoSpecialCharacters);
	}

	if issues.is_empty() {
		Ok(())
	} else {
		Err(issues)
	}
}

pub fn validate_username<'a>(username: &'a str) -> Result<(), Vec<UsernameError>> {
	let mut issues = Vec::new();

	if username.len() < 3 {
		issues.push(UsernameError::TooShort);
	}
	if username.len() > 32 {
		issues.push(UsernameError::TooLong);
	}
	if username.chars().any(|c| !c.is_ascii_alphanumeric()) {
		issues.push(UsernameError::SpecialCharacters);
	}

	if issues.is_empty() {
		Ok(())
	} else {
		Err(issues)
	}
}

pub fn validate_email<'a>(email: &'a str) -> Result<(), Vec<EmailError>> {
	let mut issues = Vec::new();

	// TODO: Remove unwrap
	let email_regex = Regex::new(r"(^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$)").unwrap();
	if !email_regex.is_match(email) {
		issues.push(EmailError::WrongFormat);
	}
	if email.len() < 6 {
		issues.push(EmailError::TooShort);
	}
	if email.len() > 64 {
		issues.push(EmailError::TooLong);
	}

	if issues.is_empty() {
		Ok(())
	} else {
		Err(issues)
	}
}

#[cfg(test)]
mod tests {
	use crate::routes::auth::utils::{validate_email, validate_password, validate_username};

	#[test]
	fn validate_email_test() {
		assert_eq!(validate_email("name@mail").is_ok(), false);
		assert_eq!(validate_email("@mail.com").is_ok(), false);
		assert_eq!(validate_email("name@.com").is_ok(), false);
		assert_eq!(validate_email("name@mail.com").is_ok(), true);
	}

	#[test]
	fn validate_username_test() {
		assert_eq!(validate_username("!Username123").is_ok(), false);
		assert_eq!(validate_username("Us").is_ok(), false);
		assert_eq!(validate_username("Username123").is_ok(), true);
		assert_eq!(validate_username("username").is_ok(), true);
	}

	#[test]
	fn validate_password_test() {
		assert_eq!(validate_password("password").is_ok(), false);
		assert_eq!(validate_password("Password").is_ok(), false);
		assert_eq!(validate_password("Password123").is_ok(), false);
		assert_eq!(validate_password("!Password123").is_ok(), true);
	}
}
