use crate::handlers::hello_world::hello_world;
use axum::{routing::get, Router, Server};
use axum_prometheus::PrometheusMetricLayer;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;

mod handlers;

#[derive(Clone)]
struct AppState {
    _db_pool: sqlx::PgPool,
}

#[tokio::main]
async fn main() {
    dotenv().expect("No .env file found");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to connect to Postgres");

    // Configure the metrics exporter
    let (prometheus_layer, metric_handler) = PrometheusMetricLayer::pair();
    let state = AppState { _db_pool: pool };

    let app = Router::new()
        .route("/", get(hello_world))
        .route("/metrics", get(|| async move { metric_handler.render() }))
        .layer(prometheus_layer)
        .with_state(state);

    Server::bind(&SocketAddr::from(([0, 0, 0, 0], 9000)))
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use axum::{routing::get, Router};
    use axum_prometheus::PrometheusMetricLayer;
    use tower::ServiceExt;

    // reference from https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs
    #[tokio::test]
    async fn main() {
        let (prometheus_layer, metric_handler) = PrometheusMetricLayer::pair();

        let app = Router::new()
            .route("/", get(hello_world))
            .route("/metrics", get(|| async move { metric_handler.render() }))
            .layer(prometheus_layer);

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(&body[..], b"Hello World");
    }
}
