use crate::handlers::hello_world::hello_world;
use axum::{routing::get, Router, Server};
use axum_prometheus::metrics_exporter_prometheus::PrometheusHandle;
use axum_prometheus::PrometheusMetricLayer;
use std::net::SocketAddr;

mod handlers;

#[tokio::main]
async fn main() {
    let (prometheus_layer, metric_handler) = PrometheusMetricLayer::pair();
    let app = app(metric_handler);
    let app = app.layer(prometheus_layer);

    let addr = SocketAddr::from(([0, 0, 0, 0], 9000));

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn app(metric_handler: PrometheusHandle) -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/metrics", get(|| async move { metric_handler.render() }))
}

#[cfg(test)]
mod tests {
    use super::app;
    use axum::body::Body;

    use axum::http::{Request, StatusCode};
    use axum_prometheus::PrometheusMetricLayer;
    use tower::ServiceExt;

    // reference from https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs
    #[tokio::test]
    async fn main() {
        let (prometheus_layer, metric_handler) = PrometheusMetricLayer::pair();
        let app = app(metric_handler);
        let app = app.layer(prometheus_layer);

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(&body[..], b"Hello World");
    }
}
