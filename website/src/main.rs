mod content;
mod markdown;
mod routes;

use axum::{routing::get, Router};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::{compression::CompressionLayer, services::ServeDir, trace::TraceLayer};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,tower_http=info".into()),
        )
        .init();

    // BW_SITE_ROOT lets the binary find templates/content/static regardless of CWD.
    let root = PathBuf::from(std::env::var("BW_SITE_ROOT").unwrap_or_else(|_| ".".into()));

    // BW_WRITINGS_ROOT is the repo root that holds the canonical `essays/` and
    // `assets/` — the writing "data". The site renders these directly; there is
    // no duplicated copy. It defaults to BW_SITE_ROOT so the flattened Docker
    // image works with one path; for local dev (running inside `website/`) point
    // it at the repo root, e.g. `BW_WRITINGS_ROOT=..`.
    let writings_root = std::env::var("BW_WRITINGS_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| root.clone());

    let cfg = content::load_site(&root).expect("failed to load content/site.toml");
    let writings = content::load_writings(&root).expect("failed to load content/writings.toml");
    let mut env = content::build_env(&root).expect("failed to load templates/");
    env.add_global("asset_version", content::asset_version(&root));

    let state = routes::AppState {
        cfg: Arc::new(cfg),
        writings: Arc::new(writings),
        env: Arc::new(env),
        root: Arc::new(root.clone()),
        writings_root: Arc::new(writings_root.clone()),
    };

    let app = Router::new()
        .route("/", get(routes::home))
        .route("/writings", get(routes::writings_index))
        .route("/writings/:slug", get(routes::essay))
        .route("/consulting", get(routes::consulting))
        .route("/resume", get(routes::resume))
        .route("/healthz", get(|| async { "ok" }))
        .nest_service("/static", ServeDir::new(root.join("static")))
        .nest_service("/assets", ServeDir::new(writings_root.join("assets")))
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind");
    tracing::info!("Buddy Williams site listening on http://{addr}");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("server error");
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
    tracing::info!("shutting down");
}
