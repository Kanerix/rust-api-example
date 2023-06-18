
/// JWT keys used for signing and verifying tokens
static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
	Keys::new(secret.as_bytes())
});

/// JWT keys
struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

/// Generates a refresh token
pub fn generate_refresh_token(length: usize) -> String {
	let rng = thread_rng();
	rng.sample_iter(&Alphanumeric)
		.take(length)
		.map(char::from)
		.collect()
}

/// Generates an access token
pub fn generate_access_token(user: &User) -> anyhow::Result<String> {
	let claims = Claims {
		sub: user.id.to_string(),
		iss: "auth.artilun.com".to_string(),
		aud: "www.artilun.com".to_string(),
        exp: (chrono::Utc::now().timestamp() + 3600) as usize,
        nbf: chrono::Utc::now().timestamp() as usize,
        iat: chrono::Utc::now().timestamp() as usize,
		username: user.username.clone(),
		role: user.role.clone(),
	};

	let token = encode(
		&Header::new(Algorithm::EdDSA),
		&claims,
		&KEYS.encoding,
	)?;

	Ok(token)
}

/// Verifies an access token
pub fn verify_access_token(token: &str) -> anyhow::Result<TokenData<Claims>> {
	let token = decode::<Claims>(
		token,
		&KEYS.decoding,
		&Validation::new(Algorithm::EdDSA),
	)?;

	Ok(token)
}
