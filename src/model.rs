use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct User {
	pub id: i32,
	pub email: String,
	pub username: String,
	pub password: String,
	pub role: Role,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

#[derive(sqlx::Type, Serialize, Deserialize, PartialEq, PartialOrd, Debug, Clone)]
#[sqlx(type_name = "role", rename_all = "lowercase")]
pub enum Role {
	ADMIN,
	USER,
}

impl std::str::FromStr for Role {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ADMIN" => Ok(Role::ADMIN),
            "USER" => Ok(Role::USER),
            _ => Err(()),
        }
    }
}