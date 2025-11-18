use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::services::SpotifyService;

#[derive(Clone)]
pub struct AppState {
    pub spotify: Arc<SpotifyService>,
}

/// Response for login endpoint
#[derive(Serialize)]
pub struct LoginResponse {
    pub auth_url: String,
}

/// Callback query parameters from Spotify
#[derive(Deserialize)]
pub struct CallbackParams {
    pub code: Option<String>,
    pub error: Option<String>,
}

/// Status response
#[derive(Serialize)]
pub struct StatusResponse {
    pub authenticated: bool,
}

/// Client URL helper function
fn get_client_url() -> String{
    std::env::var("CLIENT_URL")
        .unwrap_or_else(|_| "http://localhost:8080".to_string())
}

/// Initiate Spotify login
pub async fn login(
    State(state): State<AppState>,
) -> Result<Redirect, (StatusCode, String)> {
    // Check if already authenticated
    if state.spotify.is_authenticated().await {
        tracing::info!("Already authenticated, redirecting to dashboard");
        let client_url = get_client_url();
        // Redirect to client dashboard (adjust port as needed)
        return Ok(Redirect::to(&client_url));
    }

    let auth_url = state
        .spotify
        .get_auth_url()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Redirect::to(&auth_url))
}

/// Handle Spotify OAuth callback
pub async fn callback(
    State(state): State<AppState>,
    Query(params): Query<CallbackParams>,
) -> Result<Redirect, (StatusCode, String)> {
    // Check for error from Spotify
    if let Some(error) = params.error {
        tracing::error!("Spotify authorization error: {}", error);
        return Err((
            StatusCode::UNAUTHORIZED,
            format!("Authorization failed: {}", error),
        ));
    }

    // Get authorization code
    let code = params
        .code
        .ok_or((StatusCode::BAD_REQUEST, "No code provided".to_string()))?;

    // Exchange code for token
    state
        .spotify
        .authenticate(&code)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Redirect to client app
    let client_url = get_client_url();
    Ok(Redirect::to(&client_url))
}

/// Check authentication status
pub async fn status(
    State(state): State<AppState>,
) -> Json<StatusResponse> {
    let authenticated = state.spotify.is_authenticated().await;
    Json(StatusResponse { authenticated })
}

/// Test authentication API endpoint
 pub async fn test_api(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)>{
    // Check if authenticated
    if !state.spotify.is_authenticated().await{
        return Err((
            StatusCode::UNAUTHORIZED,
            "Not authenticated with Spotify".to_string(),
        ));
    }

    Ok(Json(serde_json::json!({
        "message": "Authentication is working!",
        "authenticated": true
    })))
}