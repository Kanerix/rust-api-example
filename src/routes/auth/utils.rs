use argon2::{Config, ThreadMode};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, TokenData, Algorithm};
use once_cell::sync::Lazy;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::{Serialize, Deserialize};

use crate::model::{User, Role};

/// Config used for hashing passwords
static CONFIG: Lazy<Config> = Lazy::new(|| {
	let secret = std::env::var("HASH_PEPPER").expect("HASH_PEPPER must be set");
	Config {
		ad: &[],
		hash_length: 32,
		mem_cost: 4096,
		time_cost: 10,
		lanes: 4,
		thread_mode: ThreadMode::Parallel,
		variant: argon2::Variant::Argon2id,
		version: argon2::Version::Version13,
		secret: &secret.as_bytes(),
	}
});

/// Hash password
pub fn hash_password(password: &[u8]) -> anyhow::Result<String> {
	let salt = std::env::var("HASH_SALT").expect("HASH_SALT must be set");
	let password_hash = argon2::hash_encoded(
		password,
		&salt.as_bytes(),
		&CONFIG
	)?;

	Ok(password_hash)
}

pub fn verify_password(password: &[u8]) {
	let salt = std::env::var("HASH_SALT").expect("HASH_SALT must be set");
	let password_hash = argon2::hash_encoded(
		password,
		&salt.as_bytes(),
		&CONFIG
	)?;
}


/// JWT claims
#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
	pub sub: String,
	pub iss: String,
	pub aud: String,
	pub exp: usize,
	pub nbf: usize,
	pub iat: usize,
	pub username: String,
	pub role: Role,
}

