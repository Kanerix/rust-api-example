use argon2::{Config, ThreadMode};
use once_cell::sync::Lazy;

static PEPPER: Lazy<Vec<u8>> = Lazy::new(|| {
	std::env::var("HASH_PEPPER")
		.expect("HASH_PEPPER must be set")
		.as_bytes()
		.into()
});

static SALT: Lazy<Vec<u8>> = Lazy::new(|| {
	std::env::var("HASH_SALT")
		.expect("HASH_SALT must be set")
		.as_bytes()
		.into()
});

/// Hash password
pub fn hash_password(password: &[u8]) -> anyhow::Result<String> {
	let password_hash = argon2::hash_encoded(password, &SALT, &CONFIG)?;

	Ok(password_hash)
}

/// Config used for hashing passwords
static CONFIG: Lazy<Config> = Lazy::new(|| Config {
	ad: &[],
	hash_length: 32,
	mem_cost: 4096,
	time_cost: 10,
	lanes: 4,
	secret: PEPPER.as_ref(),
	thread_mode: ThreadMode::Parallel,
	variant: argon2::Variant::Argon2id,
	version: argon2::Version::Version13,
});

pub fn verify_password(password_hash: &str, password: &[u8]) -> anyhow::Result<bool> {
	let matches = argon2::verify_encoded(password_hash, password)?;

	Ok(matches)
}
