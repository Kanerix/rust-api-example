use std::fmt;

pub enum PasswordError {
	TooShort,
	TooLong,
	NoSpecialCharacters,
	NoUppercase,
	NoLowercase,
	NoNumbers,
}

impl fmt::Display for PasswordError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PasswordError::TooShort => write!(f, "TooShort"),
			PasswordError::TooLong => write!(f, "TooLong"),
			PasswordError::NoSpecialCharacters => write!(f, "NoSpecialCharacters"),
			PasswordError::NoUppercase => write!(f, "NoUppercase"),
			PasswordError::NoLowercase => write!(f, "NoLowercase"),
			PasswordError::NoNumbers => write!(f, "NoNumbers"),
        }
	}
}

pub fn validate_password<'a>(password: &'a str) -> Result<(), PasswordError> {
	if password.len() < 8 {
		return Err(PasswordError::TooShort);
	} else if password.len() > 64 {
		return Err(PasswordError::TooLong);
	} else if !password.chars().any(|c| c.is_ascii_uppercase()) {
		return Err(PasswordError::NoUppercase);
	} else if !password.chars().any(|c| c.is_ascii_lowercase()) {
		return Err(PasswordError::NoLowercase);
	} else if !password.chars().any(|c| c.is_ascii_digit()) {
		return Err(PasswordError::NoNumbers);
	} else if !password.chars().any(|c| !c.is_ascii_alphanumeric()) {
		return Err(PasswordError::NoSpecialCharacters);
	}
	Ok(())
}

pub enum UsernameError {

}

pub fn validate_username() -> Result<(), UsernameError> {
	Ok(())
}

pub enum EmailError {

}

pub fn validate_email() -> Result<(), EmailError> {
	Ok(())
}