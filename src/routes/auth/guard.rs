use axum::{
	body::BoxBody,
	http::{Request, Response},
	middleware::Next,
};

use crate::problem::Problem;

pub async fn guard<T>(request: Request<T>, next: Next<T>) -> Result<Response<BoxBody>, Problem> {
	let headers = request.headers();
	// println!("{:?}", headers);

	Ok(next.run(request).await)
}
