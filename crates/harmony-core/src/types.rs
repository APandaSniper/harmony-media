use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================================
// Authentication Types
// ============================================================================

/// Spotify OAuth authentication state
///
/// Stores the access token, optional refresh token, and expiration time
/// for maintaining authenticated sessions with the Spotify API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotifyAuthState {
    /// OAuth access token for API requests
    pub access_token: String,

    /// Optional refresh token for obtaining new access tokens
    pub refresh_token: Option<String>,

    /// Unix timestamp when the access token expires
    pub expires_at: i64,
}

// ============================================================================
// Track Types
// ============================================================================

/// Universal track identifier
///
/// Harmony's internal unique identifier for tracks across all providers.
/// Wraps a UUID v4 for guaranteed uniqueness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TrackId(pub Uuid);

impl TrackId {
    /// Generate a new unique track identifier
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Parse a track ID from a string representation
    ///
    /// # Arguments
    /// * `s` - UUID string in standard format (e.g., "550e8400-e29b-41d4-a716-446655440000")
    ///
    /// # Returns
    /// * `Some(TrackId)` if parsing succeeds
    /// * `None` if the string is not a valid UUID
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

/// Lightweight track reference for UI display and queueing
///
/// Contains minimal metadata needed to display a track in playlists
/// and UI components without loading full track details. Similar to
/// Spotify's `SimplifiedTrack` or Apple Music's track metadata.
///
/// # Performance
/// Use `TrackRef` for:
/// - Displaying tracks in playlist views
/// - Building play queues
/// - Showing search results
///
/// Convert to full `Track` only when needed for playback or detailed views.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackRef {
    /// Harmony's internal ID (if this track was previously converted to full `Track`)
    ///
    /// `None` indicates this track hasn't been fully processed yet.
    /// Useful for linking lightweight refs back to cached full tracks.
    pub harmony_id: Option<TrackId>,

    /// Provider-specific track identifier
    ///
    /// Examples:
    /// - Spotify: Track ID like "3n3Ppam7vgaVa1iaRUc9Lp"
    /// - Local: File path like "/music/song.mp3"
    /// - YouTube: Video ID like "dQw4w9WgXcQ"
    pub provider_id: String,

    /// Media provider name
    ///
    /// Values: "spotify", "local", "youtube", "apple_music", etc.
    pub provider: String,

    /// Track title/name
    pub name: String,

    /// List of artist names
    ///
    /// Multiple artists are common for collaborations and features.
    /// Order typically reflects primary artist first, then featured artists.
    pub artists: Vec<String>,

    /// Track duration in milliseconds
    ///
    /// Standard audio duration format. Use milliseconds for precision
    /// and consistency with provider APIs (Spotify, Apple Music use ms).
    pub duration_ms: u32,

    /// Optional URL to album artwork
    ///
    /// May be `None` if:
    /// - Track is still being processed
    /// - Provider doesn't provide artwork
    /// - Artwork hasn't been fetched yet
    pub artwork_url: Option<String>,
}

/// Full track with complete metadata
///
/// Represents a track that has been fully converted to Harmony's internal
/// format with all available metadata. Use this for:
/// - Active playback
/// - Detailed track information views
/// - Music library management
/// - Data that needs to be persisted
///
/// # Provider Agnostic
/// Normalizes track data from any provider (Spotify, local files, YouTube, etc.)
/// into a unified format for consistent handling throughout the application.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    /// Harmony's internal unique identifier
    pub harmony_id: TrackId,

    /// Provider-specific track identifier
    ///
    /// Format varies by provider:
    /// - Spotify: "spotify:track:3n3Ppam7vgaVa1iaRUc9Lp"
    /// - Local: Full file path
    /// - YouTube: Video URL or ID
    pub provider_id: String,

    /// Media provider name
    ///
    /// Values: "spotify", "local", "youtube", "apple_music", "harmony"
    pub provider: String,

    /// Track title
    pub title: String,

    /// List of artist names
    ///
    /// Ordered from primary artist to featured/collaborating artists
    pub artist: Vec<String>,

    /// Album name
    ///
    /// `None` for:
    /// - Singles without album association
    /// - Tracks where album info is unavailable
    /// - Non-album content (podcasts, audiobooks)
    pub album: Option<String>,

    /// Track duration in seconds (floating point for precision)
    ///
    /// Uses seconds (not milliseconds) for compatibility with audio
    /// playback libraries that expect fractional seconds.
    pub duration: f64,

    /// URL to 30-second preview clip
    ///
    /// Primarily used by Spotify. Most providers offer preview clips
    /// for non-authenticated playback or trial purposes.
    pub preview_url: Option<String>,

    /// URL to full track for streaming/download
    ///
    /// Format depends on provider:
    /// - Spotify: spotify:track URI or web player URL
    /// - YouTube: Full video URL
    /// - Local: file:// URL
    pub full_url: Option<String>,

    /// URL or file path to album artwork
    ///
    /// Typically the highest quality artwork available from the provider
    pub artwork_url: Option<String>,

    /// Release date as Unix timestamp
    ///
    /// Precision varies by provider:
    /// - Some provide exact date (YYYY-MM-DD)
    /// - Some provide year only (YYYY)
    /// - Some provide year-month (YYYY-MM)
    ///
    /// `None` if release date is unavailable
    pub release_date: Option<i64>,

    /// Upload/import date as Unix timestamp
    ///
    /// When this track was added to Harmony, not the original release date
    pub upload_date: i64,

    /// Categorization tags
    ///
    /// May include:
    /// - Genres (rock, jazz, electronic)
    /// - Moods (energetic, calm, dark)
    /// - Themes (workout, study, party)
    /// - Custom user tags
    pub tags: Option<Vec<String>>,
}

impl Track {
    /// Create a minimal track with required fields only
    ///
    /// Useful for testing or creating placeholder tracks.
    /// All optional fields are set to `None` or default values.
    ///
    /// # Arguments
    /// * `title` - Track title
    /// * `artist` - Primary artist name
    ///
    /// # Example
    /// ```
    /// let track = Track::minimal("Bohemian Rhapsody", "Queen");
    /// ```
    pub fn minimal(title: impl Into<String>, artist: impl Into<String>) -> Self {
        Self {
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

    /// Convert to lightweight track reference
    /// 
    /// Creates a `TrackRef` containing essential display information
    /// without the full metadata. Useful for:
    /// - Adding to playlists after full conversion
    /// - Building play queues from cached tracks
    /// - Displaying track lists efficiently
    /// 
    /// # Returns
    /// `TrackRef` with core track information
    /// 
    /// # Example
    /// ```
    /// let full_track = convert_spotify_track(spotify_track)?;
    /// let track_ref = full_track.to_ref();
    /// playlist.items.push(PlaylistItem::Track(track_ref));
    /// ```
    pub fn to_ref(&self) -> TrackRef {
        TrackRef {
            harmony_id: Some(self.harmony_id),
            provider_id: self.provider_id.clone(),
            provider: self.provider.clone(),
            name: self.title.clone(),
            artists: self.artist.clone(),
            duration_ms: (self.duration * 1000.0) as u32,
            artwork_url: self.artwork_url.clone(),
        }
    }
}

// ============================================================================
// Track Group Types
// ============================================================================

/// Universal group identifier
///
/// Harmony's internal unique identifier for track groups.
/// Track groups are Harmony-native collections that can be inserted
/// into playlists as single units.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GroupId(pub Uuid);

impl GroupId {
    /// Generate a new unique group identifier
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Parse a group ID from a string representation
    ///
    /// # Arguments
    /// * `s` - UUID string in standard format
    ///
    /// # Returns
    /// * `Some(GroupId)` if parsing succeeds
    /// * `None` if the string is not a valid UUID
    pub fn from_string(s: &str) -> Option<Self> {
        Uuid::parse_str(s).ok().map(Self)
    }
}

impl Default for GroupId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for GroupId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Lightweight track group reference for UI display
///
/// Contains summary information about a track group without loading
/// all individual tracks. Use this for:
/// - Displaying groups in playlist views
/// - Showing group cards in a group library
/// - Quick access to group metadata
///
/// Load the full `TrackGroup` only when needed (e.g., user expands
/// the group or starts playback).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupRef {
    /// Harmony's internal group identifier
    pub harmony_id: GroupId,

    /// User-friendly group name
    pub name: String,

    /// Number of tracks in this group
    pub track_count: usize,

    /// Combined duration of all tracks in milliseconds
    pub total_duration_ms: u32,

    /// Group artwork URL
    ///
    /// Typically defaults to the first track's artwork if not
    /// explicitly set by the user
    pub artwork_url: Option<String>,
}

/// A reusable collection of tracks that plays as a unit
///
/// Track groups are a Harmony-exclusive feature that allows users to:
/// - Create multi-track units that always play together
/// - Insert the same group into multiple playlists
/// - Edit a group once and have changes reflect everywhere
/// - Optionally shuffle tracks within the group while keeping the group intact
///
/// # Use Cases
/// - Album medleys (e.g., Abbey Road Side B)
/// - Multi-movement classical pieces
/// - DJ mix transitions
/// - Workout sequences
/// - Story arcs in concept albums
///
/// # Example
/// ```
/// // Create a group for Pink Floyd's "The Wall" Act 1
/// let mut wall_act1 = TrackGroup::new("The Wall - Act 1", "user@example.com");
/// wall_act1.add_track(in_the_flesh_ref);
/// wall_act1.add_track(thin_ice_ref);
/// wall_act1.add_track(another_brick_ref);
/// wall_act1.allow_shuffle = false; // Must play in order
///
/// // Use in multiple playlists
/// playlist1.items.push(PlaylistItem::Group(wall_act1.to_ref()));
/// playlist2.items.push(PlaylistItem::Group(wall_act1.to_ref()));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackGroup {
    /// Harmony's internal unique identifier
    pub harmony_id: GroupId,

    /// User-friendly group name
    ///
    /// Should be descriptive (e.g., "Abbey Road Medley" not "Group 1")
    pub name: String,

    /// Optional detailed description
    ///
    /// Can explain the group's purpose, context, or special instructions
    /// Example: "Play these 5 tracks for a 10-minute warm-up sequence"
    pub description: Option<String>,

    /// Ordered list of tracks in this group
    ///
    /// Order matters! Tracks play in the sequence defined here
    /// (unless `allow_shuffle` is true)
    pub tracks: Vec<TrackRef>,

    /// Whether tracks within this group can be shuffled
    ///
    /// - `false` (default): Tracks must play in defined order
    /// - `true`: Tracks can be shuffled within the group
    ///
    /// Note: The group itself always plays as a unit in playlists,
    /// this only affects internal track order
    pub allow_shuffle: bool,

    /// Custom group artwork
    ///
    /// If `None`, defaults to the first track's artwork.
    /// Users can set custom artwork for better visual organization.
    pub artwork_url: Option<String>,

    /// Creation timestamp (Unix time)
    pub created_at: i64,

    /// Last modification timestamp (Unix time)
    ///
    /// Updated whenever tracks are added/removed/reordered or
    /// group metadata changes
    pub updated_at: i64,

    /// Owner/creator user identifier
    ///
    /// Typically a user ID or email address
    pub owner: String,

    /// Categorization tags
    ///
    /// Examples: "workout", "classical", "album-medley", "favorites"
    pub tags: Option<Vec<String>>,
}

impl TrackGroup {
    /// Create a new empty track group
    ///
    /// Initializes a group with sensible defaults:
    /// - No tracks
    /// - Shuffle disabled (must play in order)
    /// - No custom artwork
    /// - Created/updated timestamps set to now
    ///
    /// # Arguments
    /// * `name` - Group name (should be descriptive)
    /// * `owner` - User identifier who owns this group
    ///
    /// # Example
    /// ```
    /// let group = TrackGroup::new("Morning Workout", "user@example.com");
    /// ```
    pub fn new(name: impl Into<String>, owner: impl Into<String>) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            harmony_id: GroupId::new(),
            name: name.into(),
            description: None,
            tracks: Vec::new(),
            allow_shuffle: false,
            artwork_url: None,
            created_at: now,
            updated_at: now,
            owner: owner.into(),
            tags: None,
        }
    }

    /// Add a track to the end of this group
    ///
    /// Updates the `updated_at` timestamp automatically.
    ///
    /// # Arguments
    /// * `track` - Track reference to add
    ///
    /// # Example
    /// ```
    /// group.add_track(track_ref);
    /// ```
    pub fn add_track(&mut self, track: TrackRef) {
        self.tracks.push(track);
        self.updated_at = chrono::Utc::now().timestamp();
    }

    /// Calculate total duration of all tracks in the group
    ///
    /// # Returns
    /// Sum of all track durations in milliseconds
    pub fn total_duration_ms(&self) -> u32 {
        self.tracks.iter().map(|t| t.duration_ms).sum()
    }

    /// Get number of tracks in this group
    ///
    /// # Returns
    /// Track count
    pub fn track_count(&self) -> usize {
        self.tracks.len()
    }

    /// Convert to lightweight reference
    ///
    /// Creates a `GroupRef` containing summary information without
    /// the full track list. Useful for displaying groups without
    /// loading all track data.
    ///
    /// # Returns
    /// `GroupRef` with group metadata
    ///
    /// # Example
    /// ```
    /// let group_ref = group.to_ref();
    /// playlist_items.push(PlaylistItem::Group(group_ref));
    /// ```
    pub fn to_ref(&self) -> GroupRef {
        GroupRef {
            harmony_id: self.harmony_id,
            name: self.name.clone(),
            track_count: self.track_count(),
            total_duration_ms: self.total_duration_ms(),
            artwork_url: self
                .artwork_url
                .clone()
                .or_else(|| self.tracks.first()?.artwork_url.clone()),
        }
    }

    /// Get artwork URL with fallback to first track
    ///
    /// Returns custom artwork if set, otherwise falls back to
    /// the first track's artwork.
    ///
    /// # Returns
    /// - `Some(url)` if custom artwork exists or first track has artwork
    /// - `None` if no artwork is available
    pub fn get_artwork(&self) -> Option<String> {
        self.artwork_url
            .clone()
            .or_else(|| self.tracks.first()?.artwork_url.clone())
    }
}

// ============================================================================
// Playlist Types
// ============================================================================

/// Represents an item in a playlist
///
/// Playlists can contain either individual tracks or entire track groups.
/// This enables flexible playlist organization:
/// - Mix individual tracks with multi-track groups
/// - Reuse groups across multiple playlists
/// - Maintain group integrity even when playlist is shuffled
///
/// # Example Playlist Structure
/// ```
/// Playlist: "Sunday Morning"
/// 1. Track: "Here Comes The Sun"
/// 2. Group: "Abbey Road Medley" (8 tracks)
/// 3. Track: "Blackbird"
/// 4. Track: "Let It Be"
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlaylistItem {
    /// A single track reference
    Track(TrackRef),

    /// A track group reference
    ///
    /// Contains group metadata but not individual tracks.
    /// Load full `TrackGroup` when needed for playback or display.
    Group(GroupRef),
}

/// Universal playlist identifier
///
/// Harmony's internal unique identifier for playlists across all providers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlaylistId(pub Uuid);

impl PlaylistId {
    /// Generate a new unique playlist identifier
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Parse a playlist ID from a string representation
    ///
    /// # Arguments
    /// * `s` - UUID string in standard format
    ///
    /// # Returns
    /// * `Some(PlaylistId)` if parsing succeeds
    /// * `None` if the string is not a valid UUID
    pub fn from_string(s: &str) -> Option<Self> {
        Uuid::parse_str(s).ok().map(Self)
    }
}

impl Default for PlaylistId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for PlaylistId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// System-agnostic playlist that can contain content from any provider
///
/// Playlists can originate from external providers (Spotify, Apple Music)
/// or be created natively in Harmony. This unified structure allows:
/// - Syncing playlists from streaming services
/// - Creating custom Harmony playlists
/// - Mixing tracks from multiple providers in one playlist
/// - Lazy loading of playlist contents for performance
///
/// # Provider Types
/// - `"harmony"` - Created in Harmony, fully editable
/// - `"spotify"` - Synced from Spotify, editable on Spotify only
/// - `"apple_music"` - Synced from Apple Music
/// - `"local"` - Local file-based playlist (m3u, etc.)
///
/// # Lazy Loading
/// The `items` field is `Option<Vec<PlaylistItem>>`:
/// - `None` - Items not loaded yet (just metadata)
/// - `Some([...])` - Items loaded and ready for display
///
/// This enables fast initial playlist listing without loading
/// thousands of tracks upfront.
///
/// # Example
/// ```
/// // Create native Harmony playlist
/// let mut playlist = Playlist::new_native("My Mix", "user@example.com");
///
/// // Add items
/// playlist.items = Some(vec![
///     PlaylistItem::Track(track_ref),
///     PlaylistItem::Group(group_ref),
/// ]);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    /// Harmony's internal unique identifier
    pub harmony_id: PlaylistId,

    /// Provider where this playlist originated
    ///
    /// Values: "harmony", "spotify", "apple_music", "local", etc.
    pub provider: String,

    /// Provider's specific playlist identifier
    ///
    /// Examples:
    /// - Spotify: "37i9dQZF1DXcBWIGoYBM5M"
    /// - Apple Music: Playlist ID from AM API
    /// - `None` for Harmony-native playlists
    pub provider_id: Option<String>,

    /// Playlist name/title
    pub name: String,

    /// Optional playlist description
    ///
    /// May contain user notes, theme, or purpose
    pub description: Option<String>,

    /// Optional playlist cover artwork
    ///
    /// May be custom user image or auto-generated from tracks
    pub image_url: Option<String>,

    /// Total number of tracks in playlist
    ///
    /// Includes tracks within groups. For example:
    /// - 5 individual tracks + 1 group with 10 tracks = track_count: 15
    pub track_count: u32,

    /// Owner/creator user identifier
    ///
    /// For synced playlists, this is the Harmony user who synced it
    pub owner: String,

    /// Creation timestamp in Harmony (Unix time)
    ///
    /// Not necessarily the original creation date on the provider
    pub created_at: i64,

    /// Last sync timestamp from provider (Unix time)
    ///
    /// - `Some(timestamp)` - Last successful sync time
    /// - `None` - Never synced (Harmony-native playlist)
    ///
    /// Used to determine if playlist needs refreshing from provider
    pub last_synced: Option<i64>,

    /// Whether this playlist can be edited in Harmony
    ///
    /// - `true` - Harmony-native playlist, fully editable
    /// - `false` - Synced from provider, edit on provider platform
    pub is_editable: bool,

    /// Categorization tags
    ///
    /// Examples: "workout", "chill", "party", "road-trip"
    pub tags: Option<Vec<String>>,

    /// Playlist contents (tracks and groups)
    ///
    /// - `None` - Not loaded yet (lazy loading)
    /// - `Some([...])` - Items loaded and ready
    ///
    /// Use `items_loaded()` to check status
    pub items: Option<Vec<PlaylistItem>>,
}

impl Playlist {
    /// Create a new Harmony-native playlist
    ///
    /// Initializes an empty, editable playlist with sensible defaults:
    /// - Provider: "harmony"
    /// - Empty items list (ready to add to)
    /// - Editable
    /// - No sync info (native playlist)
    ///
    /// # Arguments
    /// * `name` - Playlist name
    /// * `owner` - User identifier who owns this playlist
    ///
    /// # Returns
    /// New empty Harmony playlist
    ///
    /// # Example
    /// ```
    /// let playlist = Playlist::new_native("Workout Mix", "user@example.com");
    /// ```
    pub fn new_native(name: impl Into<String>, owner: impl Into<String>) -> Self {
        Self {
            harmony_id: PlaylistId::new(),
            provider: "harmony".to_string(),
            provider_id: None,
            name: name.into(),
            description: None,
            image_url: None,
            track_count: 0,
            owner: owner.into(),
            created_at: chrono::Utc::now().timestamp(),
            last_synced: None,
            items: Some(Vec::new()),
            is_editable: true,
            tags: None,
        }
    }

    /// Check if playlist items are loaded
    ///
    /// # Returns
    /// - `true` if items have been loaded
    /// - `false` if items need to be loaded (lazy loading)
    pub fn items_loaded(&self) -> bool {
        self.items.is_some()
    }

    /// Check if playlist needs syncing from provider
    ///
    /// Determines if enough time has passed since last sync to
    /// warrant refreshing playlist data from the provider.
    ///
    /// # Arguments
    /// * `max_age_seconds` - Maximum age before sync needed (e.g., 3600 for 1 hour)
    ///
    /// # Returns
    /// - `true` if playlist should be synced
    /// - `false` if playlist is fresh or is Harmony-native
    ///
    /// # Example
    /// ```
    /// if playlist.needs_sync(3600) {
    ///     sync_playlist_from_spotify(&playlist).await?;
    /// }
    /// ```
    pub fn needs_sync(&self, max_age_seconds: i64) -> bool {
        // Harmony-native playlists never need syncing
        if self.provider == "harmony" {
            return false;
        }

        match self.last_synced {
            Some(synced) => {
                let age = chrono::Utc::now().timestamp() - synced;
                age > max_age_seconds
            }
            None => true, // Never synced
        }
    }
}
