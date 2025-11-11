use anyhow::{anyhow, Ok, Result};
use chrono::NaiveDate;
use harmony_core::types::Track;
use rand::seq::SliceRandom;
use rspotify::{
    model::{SearchType, TrackId as SpotifyTrackId},
    prelude::*,
    ClientCredsSpotify, Credentials,
};

/// Spotify service for interacting with Spotify API
pub struct SpotifyService {
    client: ClientCredsSpotify,
}

impl SpotifyService {
    /// Creates a new Spotify service
    pub async fn new() -> Result<Self> {
        tracing::info!("Creating a new Spotify service");

        // Load credentials
        let creds = Credentials::from_env()
            .ok_or_else(|| anyhow::anyhow!("Missing Spotify credentials"))?;

        // Create client
        let client = ClientCredsSpotify::new(creds);

        // Request access token
        client.request_token().await?;

        tracing::info!("Spotify client initialization success");

        Ok(Self { client })
    }

    /// Convert Spotify full_track to Harmony_track type
    fn convert_track(&self, track: &rspotify::model::FullTrack) -> Option<Track> {
        Some(Track {
            // Generate new Harmony ID
            harmony_id: harmony_core::types::TrackId::new(),

            // Spotify track ID (returns None if missing)
            provider_id: track.id.as_ref()?.to_string(),

            // Provider name
            provider: "spotify".to_string(),

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
