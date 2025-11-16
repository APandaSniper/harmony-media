mod services;
mod routes;

use axum::{routing::get, Router};
use routes::auth::{AppState, callback, login, status};
use services::SpotifyService;
use std::{net::SocketAddr, sync::Arc};

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
            Arc::new(service)
        }
        Err(e) => {
            tracing::error!("Failed to initialize Spotify : {}", e);
            tracing::error!(
                "Make sure SPOTIFY_CLIENT_ID is set in .env"
            );
            return;
        }
    };

    // Build our application with routes
    let app = Router::new()
        .route("/", get(|| async { "Harmony Media Server" }))
        .route("/auth/login", get(login))
        .route("/auth/callback", get(callback))
        .route("/auth/status", get(status))
        .with_state(state);

    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
