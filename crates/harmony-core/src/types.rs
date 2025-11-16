use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Spotify authentication state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotifyAuthState{
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: i64, // Unix timestamp
}

/// Universal track identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TrackId(pub Uuid);

impl TrackId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_string(s: &str) -> Option<Self> {
        Uuid::parse_str(s).ok().map(Self)
    }
}

impl Default for TrackId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for TrackId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    /// Internal Harmony Track Id
    pub harmony_id: TrackId,

    /// Provider's specific Id (i.e. Spotify, URL, Apple Music, etc.)
    pub provider_id: String,

    /// Media provider (Local, Harmony, Spotify, etc.)
    pub provider: String,

    /// Track title
    pub title: String,

    /// List of artists
    pub artist: Vec<String>,

    /// Album name (Optional)
    pub album: Option<String>,

    /// Track duration in seconds
    pub duration: f64,

    /// URL to 30-sec preview, mainly a Spotify feature (Optional)
    pub preview_url: Option<String>,

    /// URL to original track, mainly used by Spotify, YouTube, etc. (Optional)
    pub full_url: Option<String>,

    /// URL or path to album artwork (Optional)
    pub artwork_url: Option<String>,

    /// Release date (Optional)
    pub release_date: Option<i64>,

    /// Upload date
    pub upload_date: i64,

    /// List of tags related to the track, include genre, themes, etc. (Optional)
    pub tags: Option<Vec<String>>,
}

impl Track{
    pub fn minimal(title: impl Into<String>, artist: impl Into<String>) -> Self {
        Self{
            harmony_id: TrackId::new(),
            provider_id: String::new(),
            provider: "unknown".to_string(),
            title: title.into(),
            artist: vec![artist.into()],
            album: None,
            duration: 0.0,
            preview_url: None,
            full_url: None,
            artwork_url: None,
            release_date: Some(0),
            upload_date: 0,
            tags: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist{
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub track_count: u32,
    pub owner: String,
}