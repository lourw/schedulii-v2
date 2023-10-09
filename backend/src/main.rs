use axum::{routing::get, Router, Server};
use std::net::SocketAddr;
use crate::handlers::hello_world::hello_world;

mod handlers;

#[tokio::main]
async fn main() {
    let app = app();
        
    let addr = SocketAddr::from(([0, 0, 0, 0], 9000));

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn app() -> Router {
    Router::new()
        .route("/", get(hello_world))
}

#[cfg(test)]
mod tests {
    use super::app;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::{ServiceExt};

    // reference from https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs
    #[tokio::test]
    async fn main() {
        let app = app();

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(&body[..], b"Hello World");
    }
}
