use axum::{async_trait, extract::FromRequest, http::{Request, header::AUTHORIZATION}, headers::Authorization};

use crate::{model::Role, problem::Problem};

pub struct Guard(pub Role);

#[async_trait]
impl<S, B> FromRequest<S, B> for Guard
where
    // these bounds are required by `async_trait`
    B: Send + 'static,
    S: Send + Sync,
{
    type Rejection = Problem;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let token = req.headers().get(AUTHORIZATION);

		Ok(Self(Role::USER))
    }
}