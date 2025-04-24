use std::{env, time::Duration};

use axum::{Router, routing::get};
use tower_http::{
    LatencyUnit,
    compression::CompressionLayer,
    timeout::TimeoutLayer,
    trace::{DefaultMakeSpan, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::{Level, error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod controller;
mod data;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::builder()
                .with_env_var("LOG_LEVEL")
                .try_from_env()
                .unwrap_or("info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let mut router = Router::new()
        .route("/", get(controller::get_index))
        .route("/genPlist", get(controller::get_gen_plist));

    if env::var("DISABLE_COMPRESSION").map_or(true, |s| s.is_empty()) {
        router = router.layer(CompressionLayer::new());
    }

    router = router.layer(TimeoutLayer::new(Duration::from_secs(10))).layer(
        TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
            .on_request(DefaultOnRequest::new())
            .on_response(
                DefaultOnResponse::new()
                    .level(Level::INFO)
                    .latency_unit(LatencyUnit::Micros),
            )
            .on_failure(DefaultOnFailure::new().level(Level::INFO)),
    );

    let listen_addr = {
        let host = env::var("HOST").unwrap_or("localhost".to_string());
        let port = env::var("PORT").unwrap_or("8080".to_string());
        format!("{}:{}", host, port)
    };

    let listener = match tokio::net::TcpListener::bind(&listen_addr).await {
        Ok(listener) => listener,
        Err(e) => {
            error!("Failed to bind to address: {}", e);
            std::process::exit(1);
        }
    };

    info!("plist-server is listening on {}", listen_addr);

    if let Err(e) = axum::serve(listener, router).await {
        error!("Failed to start server: {}", e);
        std::process::exit(1);
    }
}
