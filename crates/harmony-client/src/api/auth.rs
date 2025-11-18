//! Authentication API calls

use crate::config::api_url;
use crate::error::ClientError;
use dioxus::prelude::*;

/// Response from the auth status endpoint
#[derive(Debug, serde::Deserialize)]
pub struct AuthStatusResponse {
    pub authenticated: bool,
}

/// Check if the user is authenticated with Spotify
pub async fn check_auth_status(
    mut auth_status: Signal<bool>, 
    mut response_text: Signal<String>
) {
    let url = format!("{}/auth/status", api_url());
    
    response_text.set("⏳ Checking authentication status...".to_string());
    
    match reqwest::get(&url).await {
        Ok(response) => {
            if !response.status().is_success() {
                let error = ClientError::ServerError(
                    response.status().as_u16(),
                    "Unexpected status code".to_string()
                );
                response_text.set(error.display_message());
                return;
            }
            
            match response.json::<AuthStatusResponse>().await {
                Ok(data) => {
                    auth_status.set(data.authenticated);
                    
                    if data.authenticated {
                        response_text.set(
                            "✅ Success!\n\nYou are authenticated with Spotify.".to_string()
                        );
                    } else {
                        response_text.set(
                            "ℹ️ Not Authenticated\n\n\
                             Click 'Login with Spotify' to connect your account.".to_string()
                        );
                    }
                }
                Err(e) => {
                    let error = ClientError::ParseError(format!("{}", e));
                    response_text.set(error.display_message());
                }
            }
        }
        Err(e) => {
            let error = ClientError::from_reqwest(e);
            response_text.set(error.display_message());
            auth_status.set(false);
        }
    }
}

/// Initiate Spotify OAuth login
pub async fn login() {
    let url = format!("{}/auth/login", api_url());
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        match webbrowser::open(&url) {
            Ok(_) => tracing::info!("Opened browser for authentication"),
            Err(e) => tracing::error!("Failed to open browser: {}", e),
        }
    }
    
    #[cfg(target_arch = "wasm32")]
    {
        use web_sys::window;
        
        if let Some(window) = window() {
            if let Err(e) = window.location().set_href(&url) {
                web_sys::console::error_1(&format!("Failed to navigate: {:?}", e).into());
            }
        }
    }
}

/// Check server health
pub async fn check_health() -> Result<serde_json::Value, ClientError> {
    let url = format!("{}/health", api_url());
    
    let response = reqwest::get(&url)
        .await
        .map_err(ClientError::from_reqwest)?;
    
    if !response.status().is_success() {
        return Err(ClientError::ServerError(
            response.status().as_u16(),
            "Health check failed".to_string()
        ));
    }
    
    response.json()
        .await
        .map_err(|e| ClientError::ParseError(format!("{}", e)))
}