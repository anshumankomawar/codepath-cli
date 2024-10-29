mod router;
mod util;

use crate::router::build_router;

#[tokio::main]
async fn main() {
    let router = build_router().expect("Failed to build router");
    let addr = String::from("0.0.0.0:3000");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::debug!("Listening on {}", addr);
    axum::serve(listener, router).await.unwrap();
}
