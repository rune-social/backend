#![deny(unsafe_code)]
#![deny(clippy::expect_used)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::panic)]
#![deny(clippy::indexing_slicing)]

use std::net::SocketAddr;

use axum::{response::IntoResponse, routing::get, Router, Server};

#[tokio::main]
async fn main() {
	dotenvy::dotenv().ok();

	tracing_subscriber::fmt::init();

	let app = Router::new().route("/", get(root));

	let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
	let server = Server::bind(&addr).serve(app.into_make_service()).await;
}

async fn root() -> impl IntoResponse {
	"Hello, World!"
}
