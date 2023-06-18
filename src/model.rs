use chrono::{Utc, DateTime};
use serde::{Serialize, Deserialize};

#[derive(sqlx::FromRow, Debug)]
pub struct User {
	pub id: i32,
	pub email: String,
	pub username: String,
	pub password: String,
	pub role: Role,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

#[derive(sqlx::Type, Serialize, Deserialize, Debug, Clone)]
#[sqlx(type_name = "role", rename_all = "lowercase")] 
pub enum Role {
	User,
	Admin,
}