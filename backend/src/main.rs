use axum::{routing::get, Router, Server};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello World" }));

        
    let addr = SocketAddr::from(([0, 0, 0, 0], 9000));

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
