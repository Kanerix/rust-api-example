use sqlx::types::time::OffsetDateTime;

#[derive(sqlx::FromRow, Debug)]
pub struct User {
	pub id: i32,
	pub email: String,
	pub username: String,
	pub password: String,
	pub created_at: OffsetDateTime,
	pub updated_at: OffsetDateTime,
}
