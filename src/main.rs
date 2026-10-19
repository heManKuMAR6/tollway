//! The `tollway` binary: parse the CLI, set up logging and signals, then hand off to the
//! library. Kept deliberately thin; everything testable lives in `tollway::app`.

use clap::Parser;
use tokio::net::TcpListener;
use tollway::cli::{Cli, Command};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    match Cli::parse().command {
        Command::Serve { host, port } => {
            let listener = TcpListener::bind((host.as_str(), port)).await?;
            tollway::app::serve(listener, shutdown_signal()).await
        }
    }
}

/// Resolve on Ctrl-C or SIGTERM so in-flight requests can finish.
async fn shutdown_signal() {
    let ctrl_c = async {
        let _ = tokio::signal::ctrl_c().await;
    };
    #[cfg(unix)]
    let terminate = async {
        use tokio::signal::unix::{SignalKind, signal};
        if let Ok(mut s) = signal(SignalKind::terminate()) {
            s.recv().await;
        }
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
    tracing::info!("shutting down");
}
