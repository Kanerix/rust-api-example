use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use once_cell::sync::Lazy;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::fs;

use crate::model::{Role, User};

/// JWT keys used for signing and verifying tokens
static KEYS: Lazy<Keys> = Lazy::new(|| {
	let private_key = fs::read("keys/private.pem").expect("Failed to read keys/private.pem");
	let public_key = fs::read("keys/public.pem").expect("Failed to read keys/public.pem");

	Keys {
		encoding: EncodingKey::from_ed_pem(&private_key).unwrap(),
		decoding: DecodingKey::from_ed_pem(&public_key).unwrap(),
	}
});

/// JWT claims
#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
	pub sub: String,
	pub iss: String,
	pub aud: String,
	pub exp: usize,
	pub iat: usize,
	pub username: String,
	pub role: Role,
}

/// JWT keys
struct Keys {
	encoding: EncodingKey,
	decoding: DecodingKey,
}

/// Generates a refresh token
pub fn generate_refresh_token() -> String {
	let rng = thread_rng();
	rng.sample_iter(&Alphanumeric)
		.take(32)
		.map(char::from)
		.collect()
}

/// Generates an access token
pub fn generate_access_token(user: &User) -> anyhow::Result<String> {
	let claims = Claims {
		sub: user.id.to_string(),
		iss: "api.artilun.com".to_string(),
		aud: "artilun.com".to_string(),
		exp: (chrono::Utc::now().timestamp() + 3600) as usize,
		iat: chrono::Utc::now().timestamp() as usize,
		username: user.username.clone(),
		role: user.role.clone(),
	};

	let token = jsonwebtoken::encode(&Header::new(Algorithm::EdDSA), &claims, &KEYS.encoding)?;

	Ok(token)
}

/// Verifies an access token
pub fn verify_access_token(token: &str) -> anyhow::Result<TokenData<Claims>> {
	let token =
		jsonwebtoken::decode::<Claims>(token, &KEYS.decoding, &Validation::new(Algorithm::EdDSA))?;

	Ok(token)
}
