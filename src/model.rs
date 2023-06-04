#[derive(sqlx::FromRow, Debug)]
pub struct User {
	pub email: String,
	pub username: String,
	pub password: String,
}
