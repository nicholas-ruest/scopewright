use scopewright_service::app::{router, AppState};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "scopewright=info,tower_http=info".into()),
        )
        .json()
        .init();
    let data = std::env::var("SCOPEWRIGHT_DATA_PATH")
        .unwrap_or_else(|_| "var/scopewright/evidence.jsonl".into());
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(8080);
    let state = AppState::open(data)?;
    let listener = tokio::net::TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], port))).await?;
    tracing::info!(port, "scopewright ready");
    axum::serve(listener, router(state)).await?;
    Ok(())
}
