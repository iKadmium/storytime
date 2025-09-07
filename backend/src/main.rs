use backend::{create_app, load_settings};
use std::{net::SocketAddr, sync::Arc};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load settings
    let settings = match load_settings() {
        Ok(settings) => {
            tracing::info!("Settings loaded successfully");
            tracing::debug!("TTS API: {}", settings.tts_api);
            tracing::debug!("LLM API: {}", settings.llm_api);
            Arc::new(settings)
        }
        Err(e) => {
            tracing::error!("Failed to load settings: {}", e);
            std::process::exit(1);
        }
    };

    // Build our application
    let app = create_app(settings);

    // Run the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
