use crate::util::Result;
use axum::routing::get;
use axum::Router;

pub fn build_router() -> Result<Router> {
    Ok(Router::new().route("/", get(|| async { "Hello world!" })))
}
