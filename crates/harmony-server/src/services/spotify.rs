use anyhow::{anyhow, Ok, Result};
use chrono::NaiveDate;
use futures::stream::{StreamExt, TryStreamExt};
use harmony_core::types::{Playlist, Provider, Track};
use rand::seq::SliceRandom;
use rspotify::{
    model::{PlaylistId, SimplifiedPlaylist},
    prelude::*,
    scopes, AuthCodePkceSpotify, ClientError, Credentials, OAuth,
};
use std::{fs, sync::Arc};
use tokio::sync::RwLock;

/// Spotify service for interacting with Spotify API
pub struct SpotifyService {
    client: Arc<RwLock<AuthCodePkceSpotify>>,
    //verifier: Arc<RwLock<Option<String>>>,
    cache_path: std::path::PathBuf,
}

impl SpotifyService {
    /// Creates a new Spotify service
    pub async fn new() -> Result<Self> {
        tracing::info!("Creating a new Spotify service with PKCE authentication");

        let client_id = std::env::var("RSPOTIFY_CLIENT_ID")
            .map_err(|_| anyhow!("RSPOTIFY_CLIENT_ID was not set in the environment"))?;

        let redirect_uri = std::env::var("RSPOTIFY_REDIRECT_URI")
            .map_err(|_| anyhow!("RSPOTIFY_REDIRECT_URI was not set in the environment"))?;

        // Create credentials without secret
        let creds = Credentials::new_pkce(&client_id);

        // Setup OAuth config with requires scopes
        let oauth = OAuth {
            redirect_uri,
            scopes: scopes!("playlist-read-private", "playlist-read-collaborative"),
            ..Default::default()
        };

        let mut client = AuthCodePkceSpotify::new(creds, oauth);

        let cache_path = std::path::PathBuf::from(".spotify_token_cache.json");

        // Try to load existing token
        if cache_path.exists() {
            if let Result::Ok(token_json) = std::fs::read_to_string(&cache_path) {
                if let Result::Ok(token) = serde_json::from_str(&token_json) {
                    *client.token.lock().await.unwrap() = Some(token);
                    tracing::info!("Loaded cached Spotify token");
                }
            }
        }

        tracing::info!("Spotify PKCE client created successfully");

        Ok(Self {
            client: Arc::new(RwLock::new(client)),
            //verifier: Arc::new(RwLock::new(None)),
            cache_path,
        })
    }

    /// Save the current token to cache file
    async fn save_token(&self) -> Result<()> {
        let client = self.client.read().await;

        if let Some(token) = client.token.lock().await.unwrap().as_ref() {
            let token_json = serde_json::to_string_pretty(token)?;
            std::fs::write(&self.cache_path, token_json)?;
            tracing::info!("Saved token to cache");
        }
        Ok(())
    }

    /// Get the authorization URL for the user to visit
    pub async fn get_auth_url(&self) -> Result<String> {
        let mut client = self.client.write().await;

        let url = client
            .get_authorize_url(None)
            .map_err(|e| anyhow!("Failed to generate auth URL: {}", e))?;

        tracing::info!("Generated auth URL");
        Ok(url)
    }

    /// Exchange authorization code for the access token
    pub async fn authenticate(&self, code: &str) -> Result<()> {
        let client = self.client.write().await;

        client
            .request_token(code)
            .await
            .map_err(|e| anyhow!("Failed to exchange code for token: {}", e))?;

        tracing::info!("Successfully authenticated with Spotify");

        drop(client);
        self.save_token().await?;

        Ok(())
    }

    /// Check if the client is already authenticated
    pub async fn is_authenticated(&self) -> bool {
        let client = self.client.read().await;

        //Checking for token
        if let Some(token) = client.get_token().lock().await.unwrap().as_ref() {
            !token.is_expired()
        } else {
            false
        }
    }

    /// Convert Spotify full_track to Harmony_track type
    fn convert_track(&self, track: &rspotify::model::FullTrack) -> Option<Track> {
        Some(Track {
            // Generate new Harmony ID
            harmony_id: harmony_core::types::TrackId::new(),

            // Spotify track ID (returns None if missing)
            provider_id: track.id.as_ref()?.to_string(),

            // Provider name
            provider: Provider::SPOTIFY,

            // Track title
            title: track.name.clone(),

            // Artists - collect all artists into a Vec
            artist: track.artists.iter().map(|a| a.name.clone()).collect(),

            // Album name
            album: Some(track.album.name.clone()),

            // Duration in seconds (Spotify gives us Duration, we need f64)
            duration: track.duration.as_seconds_f64(),

            // Preview URL (30-second clip)
            preview_url: track.preview_url.clone(),

            // Full Spotify URL
            full_url: track.external_urls.get("spotify").cloned(),

            // Album artwork (get the first/largest image)
            artwork_url: track.album.images.first().map(|img| img.url.clone()),

            // Release date - parse year from album release_date
            // Spotify gives dates like "2023-10-27" or just "2023"
            release_date: track
                .album
                .release_date
                .as_ref()
                .and_then(|date| Self::parse_date_to_timestamp(date)),

            // Upload date
            upload_date: chrono::Utc::now().timestamp(),

            // Tags - genres
            tags: None,
        })
    }

    /// Parse Spotify date string to Unix timestamp
    fn parse_date_to_timestamp(date_str: &str) -> Option<i64> {
        // Try full date: YYYY-MM-DD
        if let Some(timestamp) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
            .ok()
            .and_then(|date| date.and_hms_opt(0, 0, 0))
            .map(|dt| dt.and_utc().timestamp())
        {
            return Some(timestamp);
        }

        // Try year-month: YYYY-MM -> YYYY-MM-01
        if date_str.len() == 7 {
            let padded = format!("{}-01", date_str);
            if let Some(timestamp) = NaiveDate::parse_from_str(&padded, "%Y-%m-%d")
                .ok()
                .and_then(|date| date.and_hms_opt(0, 0, 0))
                .map(|dt| dt.and_utc().timestamp())
            {
                return Some(timestamp);
            }
        }

        // Try year only: YYYY -> YYYY-01-01
        if let Some(timestamp) = date_str
            .parse::<i32>()
            .ok()
            .and_then(|year| NaiveDate::from_ymd_opt(year, 1, 1))
            .and_then(|date| date.and_hms_opt(0, 0, 0))
            .map(|dt| dt.and_utc().timestamp())
        {
            return Some(timestamp);
        }

        None
    }
}
