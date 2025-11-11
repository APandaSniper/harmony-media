mod services;

use axum::{routing::get, Router};
use services::SpotifyService;
use std::net::SocketAddr;
use tracing::error;

#[tokio::main]
async fn main() {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    // Load .env file FIRST, before tracing
    dotenvy::dotenv().ok();

    // Initialize Spotify service
    let spotify = match SpotifyService::new().await {
        Ok(service) => {
            tracing::info!("Spotify Service initialized");
            service
        }
        Err(e) => {
            tracing::error!("Failed to initialize Spotify : {}", e);
            tracing::error!(
                "Make sure SPOTIFY_CLIENT_ID and SPOTIFY_CLIENT_SECRET are set in .env"
            );
            return;
        }
    };

    // Build our application with routes
    let app = Router::new().route("/", get(|| async { "Harmony Media Server" }));

    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
