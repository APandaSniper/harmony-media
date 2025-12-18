//! Spotify-specific API calls

use crate::config::api_url;
use crate::error::ClientError;

// Placeholder for future Spotify API functions
// Example:
// pub async fn get_playlists() -> Result<Vec<Playlist>, ClientError> { ... }
// pub async fn get_tracks(playlist_id: &str) -> Result<Vec<Track>, ClientError> { ... }

/// Example: Fetch user's Spotify playlists (to be implemented)
pub async fn get_user_playlists() -> Result<Vec<String>, ClientError> {
    // This will call your server's /spotify/playlists endpoint (to be created)
    let url = format!("{}/spotify/playlists", api_url());

    let response = reqwest::get(&url)
        .await
        .map_err(ClientError::from_reqwest)?;

    if !response.status().is_success() {
        return Err(ClientError::ServerError(
            response.status().as_u16(),
            "Failed to fetch playlists".to_string(),
        ));
    }

    // For now, return empty vec - implement when server endpoint exists
    Ok(vec![])
}
