mod services;
mod routes;

use axum::{routing::get, Router, Json};
use routes::auth::{AppState, callback, login, status};
use services::SpotifyService;
use std::{net::SocketAddr, sync::Arc};
use tower_http::cors::{CorsLayer, AllowOrigin, Any};
//use http::header::{AUTHORIZATION, CONTENT_TYPE};

// Add this function before main()
async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "harmony-media-server",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

#[tokio::main]
async fn main() {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    // Load .env file FIRST, before tracing
    dotenvy::dotenv().ok();

    // Get configuration from environment
    let host = std::env::var("SERVER_HOST")
        .unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("SERVER_PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("SERVER_PORT must be a valid number");

    // Initialize Spotify service
    let spotify = match SpotifyService::new().await {
        Ok(service) => {
            tracing::info!("Spotify Service initialized");
            Arc::new(service)
        }
        Err(e) => {
            tracing::error!("Failed to initialize Spotify : {}", e);
            tracing::error!(
                "Make sure RSPOTIFY_CLIENT_ID is set in .env"
            );
            return;
        }
    };

    let state = AppState { spotify, };

    let cors = CorsLayer::new()
        .allow_origin(Any)  // For development - allows any origin
        .allow_methods(Any)  // GET, POST, etc.
        .allow_headers(Any); // Any headers

    // Uncomment for production
    /*let cors = CorsLayer::new()
        .allow_origin("https://yourdomain.com".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);*/

    // Build our application with routes
    let app = Router::new()
        .route("/", get(|| async { "Harmony Media Server" }))
        .route("/health", get(health_check))
        .route("/auth/login", get(login))
        .route("/auth/callback", get(callback))
        .route("/auth/status", get(status))
        .layer(cors)
        .with_state(state);

    // Run the server
    let addr = format!("{}:{}", host, port)
        .parse::<SocketAddr>()
        .expect("Invalid address format");

    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
