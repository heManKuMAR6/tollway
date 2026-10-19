//! The axum application: routes, shared state and the server loop.

use std::future::Future;

use axum::{Json, Router, routing::get};
use serde_json::{Value, json};
use tokio::net::TcpListener;

/// Build the tollway HTTP application.
pub fn build_app() -> Router {
    Router::new().route("/healthz", get(healthz))
}

/// Serve the application on `listener` until `shutdown` resolves, then let in-flight
/// requests finish.
pub async fn serve<F>(listener: TcpListener, shutdown: F) -> std::io::Result<()>
where
    F: Future<Output = ()> + Send + 'static,
{
    tracing::info!(addr = %listener.local_addr()?, version = crate::VERSION, "listening");
    axum::serve(listener, build_app())
        .with_graceful_shutdown(shutdown)
        .await
}

/// Liveness: the process is up and serving requests.
async fn healthz() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}
