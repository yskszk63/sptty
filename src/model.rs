use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Default)]
pub struct StartResumeAUsersPlaybackRequest {
    pub context_uri: Option<String>,
    pub uris: Option<Vec<String>>,
    pub offset: Option<()>,
    pub position_ms: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct TransferUserPlaybackRequest {
    pub device_ids: Vec<String>,
    pub play: bool,
}

/// https://developer.spotify.com/documentation/web-api/reference/#object-deviceobject
#[derive(Debug, Deserialize)]
pub struct Device {
    pub id: String,
    pub is_active: bool,
    pub is_private_session: bool,
    pub is_restricted: bool,
    pub name: String,
    #[serde(default)]
    pub r#type: String,
    pub volume_percent: u8,
}

/// https://developer.spotify.com/documentation/web-api/reference/#object-devicesobject
#[derive(Debug, Deserialize)]
pub struct Devices {
    pub devices: Vec<Device>,
}

/// https://developer.spotify.com/documentation/web-api/reference/#object-disallowsobject
#[derive(Debug, Deserialize)]
pub struct Disallows {
    pub interrupting_playback: Option<bool>,
    pub pausing: Option<bool>,
    pub resuming: Option<bool>,
    pub seeking: Option<bool>,
    pub skipping_next: Option<bool>,
    pub skipping_prev: Option<bool>,
    pub toggling_repeat_context: Option<bool>,
    pub toggling_repeat_track: Option<bool>,
    pub toggling_shuffle: Option<bool>,
    pub transferring_playback: Option<bool>,
}

/// https://developer.spotify.com/documentation/web-api/reference/#object-externalurlobject
#[derive(Debug, Deserialize)]
pub struct ExternalUrl {
    pub spotify: String,
}

/// https://developer.spotify.com/documentation/web-api/reference/#object-contextobject
#[derive(Debug, Deserialize)]
pub struct Context {
    pub external_urls: ExternalUrl,
    pub href: String,
    #[serde(default)]
    pub r#type: String,
    pub uri: String,
}

/// https://developer.spotify.com/documentation/web-api/reference/#object-simplifiedartistobject
#[derive(Debug, Deserialize)]
pub struct SimplifiedArtist {
    pub external_urls: ExternalUrl,
    pub href: String,
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub r#type: String,
    pub uri: String,
}

/// https://developer.spotify.com/documentation/web-api/reference/#object-imageobject
#[derive(Debug, Deserialize)]
pub struct Image {
    //height: Option<u64>,
    //url: String,
    //width: Option<u64>,
}

/// https://developer.spotify.com/documentation/web-api/reference/#object-albumrestrictionobject
#[derive(Debug, Deserialize)]
pub struct AlbumRestriction {
    pub reason: String,
}

/// https://developer.spotify.com/documentation/web-api/reference/#object-simplifiedalbumobject
#[derive(Debug, Deserialize)]
pub struct SimplifiedAlbum {
    pub album_group: Option<String>,
    pub album_type: String,
    pub artists: Vec<SimplifiedArtist>,
    #[serde(default)]
    pub available_markets: Vec<String>,
    pub external_urls: ExternalUrl,
    pub href: String,
    pub id: String,
    #[serde(default)]
    pub images: Vec<Image>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: String,
    pub restrictions: Option<AlbumRestriction>,
    pub total_tracks: u64,
    #[serde(default)]
    pub r#type: String,
    pub uri: String,
}

/// https://developer.spotify.com/documentation/web-api/reference/#object-followersobject
#[derive(Debug, Deserialize)]
pub struct Followers {
    pub href: Option<String>,
    pub total: u64,
}

/// https://developer.spotify.com/documentation/web-api/reference/#object-artistobject
#[derive(Debug, Deserialize)]
pub struct Artist {
    pub external_urls: ExternalUrl,
    pub followers: Option<Followers>,
    #[serde(default)]
    pub genres: Vec<String>,
    pub href: String,
    pub id: String,
    #[serde(default)]
    pub images: Vec<Image>,
    pub name: String,
    pub popularity: Option<u64>,
    #[serde(default)]
    pub r#type: String,
    pub uri: String,
}

/// https://developer.spotify.com/documentation/web-api/reference/#object-externalidobject
#[derive(Debug, Deserialize)]
pub struct ExternalId {
    pub ean: Option<String>,
    pub isrc: Option<String>,
    pub upc: Option<String>,
}

/// https://developer.spotify.com/documentation/web-api/reference/#object-trackrestrictionobject
#[derive(Debug, Deserialize)]
pub struct TrackRestriction {
    pub reason: String,
}

/// https://developer.spotify.com/documentation/web-api/reference/#object-trackobject
#[derive(Debug, Deserialize)]
pub struct Track {
    pub album: Option<SimplifiedAlbum>,
    pub artists: Vec<Artist>,
    #[serde(default)]
    pub available_markets: Vec<String>,
    pub disc_number: u64,
    pub duration_ms: u64,
    pub explicit: bool,
    pub external_ids: Option<ExternalId>,
    pub external_urls: ExternalUrl,
    pub href: String,
    pub id: String,
    #[serde(default)]
    pub is_local: bool,
    pub is_playable: Option<bool>,
    //pub linked_from:
    pub name: String,
    pub popularity: Option<u64>,
    pub preview_url: Option<String>,
    pub restrictions: Option<TrackRestriction>,
    pub track_number: u64,
    #[serde(default)]
    pub r#type: String,
    pub uri: String,
}

/// https://developer.spotify.com/documentation/web-api/reference/#object-resumepointobject
#[derive(Debug, Deserialize)]
pub struct ResumePoint {
    pub fully_played: bool,
    pub resume_position_ms: u64,
}

/// https://developer.spotify.com/documentation/web-api/reference/#object-copyrightobject
#[derive(Debug, Deserialize)]
pub struct Copyright {
    pub text: String,
    #[serde(default)]
    pub r#type: String,
}

/// https://developer.spotify.com/documentation/web-api/reference/#object-simplifiedshowobject
#[derive(Debug, Deserialize)]
pub struct SimplifiedShow {
    #[serde(default)]
    pub available_markets: Vec<String>,
    pub copyrights: Vec<Copyright>,
    pub description: String,
    pub explicit: bool,
    pub external_urls: ExternalUrl,
    pub href: String,
    pub html_description: String,
    pub id: String,
    #[serde(default)]
    pub images: Vec<Image>,
    pub is_externally_hosted: bool,
    pub languages: Vec<String>,
    pub media_type: String,
    pub name: String,
    pub publisher: String,
    #[serde(default)]
    pub r#type: String,
    pub uri: String,
}

/// https://developer.spotify.com/documentation/web-api/reference/#object-episodeobject
#[derive(Debug, Deserialize)]
pub struct Episode {
    pub audio_preview_url: String,
    pub description: String,
    pub duration_ms: u64,
    pub explicit: bool,
    pub external_urls: ExternalUrl,
    pub href: String,
    pub html_description: String,
    pub id: String,
    #[serde(default)]
    pub images: Vec<Image>,
    pub is_externally_hosted: bool,
    pub is_playable: Option<bool>,
    pub language: String,
    pub languages: Vec<String>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: String,
    pub restrictions: Option<EpisodeRestriction>,
    pub resume_point: ResumePoint,
    pub show: SimplifiedShow,
    #[serde(default)]
    pub r#type: String,
    pub uri: String,
}

/// https://developer.spotify.com/documentation/web-api/reference/#object-episoderestrictionobject
#[derive(Debug, Deserialize)]
pub struct EpisodeRestriction {
    pub reason: String,
}

// FIXME
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum TrackOrEpisode {
    #[serde(rename = "track")]
    Track {
        #[serde(flatten)]
        inner: Track,
    },
    #[serde(rename = "episode")]
    Episode {
        #[serde(flatten)]
        inner: Episode,
    },
}

/// https://developer.spotify.com/documentation/web-api/reference/#object-currentlyplayingcontextobject
#[derive(Debug, Deserialize)]
pub struct CurrentlyPlayingContext {
    pub actions: Disallows,
    pub context: Option<Context>,
    pub currently_playing_type: String,
    pub device: Device,
    pub is_playing: bool,
    pub item: TrackOrEpisode,
    pub progress_ms: Option<u64>,
    pub repeat_state: String,
    pub shuffle_state: bool,
    pub timestamp: u64,
}

/// https://developer.spotify.com/documentation/web-api/reference/#object-publicuserobject
#[derive(Debug, Deserialize)]
pub struct PublicUser {
    pub display_name: Option<String>,
    pub external_urls: ExternalUrl,
    pub followers: Option<Followers>,
    pub href: String,
    pub id: String,
    #[serde(default)]
    pub images: Vec<Image>,
    #[serde(default)]
    pub r#type: String,
    pub uri: String,
}

/// https://developer.spotify.com/documentation/web-api/reference/#object-playlisttrackobject
#[derive(Debug, Deserialize)]
pub struct PlaylistTrack {
    pub added_at: Option<String>, // FIXME timestamp
    pub added_by: Option<PublicUser>,
    #[serde(default)]
    pub is_local: bool,
    pub track: Option<TrackOrEpisode>,
}

/// https://developer.spotify.com/documentation/web-api/reference/#object-playlistobject
#[derive(Debug, Deserialize)]
pub struct Playlist {
    pub collaborative: bool,
    pub description: Option<String>,
    pub external_urls: ExternalUrl,
    pub followers: Option<Followers>,
    pub href: String,
    pub id: String,
    #[serde(default)]
    pub images: Vec<Image>,
    pub name: String,
    pub owner: PublicUser,
    pub public: bool,
    pub snapshot_id: String,
    pub tracks: Option<PlaylistTrack>,
    #[serde(default)]
    pub r#type: String,
    pub uri: String,
}

/// https://developer.spotify.com/documentation/web-api/reference/#object-pagingobject
#[derive(Debug, Deserialize)]
pub struct Paging<T> {
    pub href: String,
    pub items: Vec<T>,
    pub limit: u64,
    pub next: Option<String>,
    pub offset: u64,
    pub previous: Option<String>,
    pub total: u64,
}

/// https://developer.spotify.com/documentation/web-api/reference/#object-linkedtrackobject
#[derive(Debug, Deserialize)]
pub struct LinkedTrack {
    pub external_urls: ExternalUrl,
    pub href: String,
    pub id: String,
    #[serde(default)]
    pub r#type: String,
    pub uri: String,
}

/// https://developer.spotify.com/documentation/web-api/reference/#object-simplifiedtrackobject
#[derive(Debug, Deserialize)]
pub struct SimplifiedTrack {
    #[serde(default)]
    pub artists: Vec<SimplifiedArtist>,
    #[serde(default)]
    pub available_markets: Vec<String>,
    pub disc_number: u64,
    pub duration_ms: u64,
    pub explicit: bool,
    pub external_urls: ExternalUrl,
    pub href: String,
    pub id: String,
    pub is_local: bool,
    pub is_playable: bool,
    pub linked_from: LinkedTrack,
    pub name: String,
    pub preview_url: String,
    pub restrictions: Option<TrackRestriction>,
    pub track_number: u64,
    #[serde(default)]
    pub r#type: String,
    pub uri: String,
}

/// https://developer.spotify.com/documentation/web-api/reference/#object-explicitcontentsettingsobject
#[derive(Debug, Deserialize)]
pub struct ExplicitContentSettings {
    pub filter_enabled: bool,
    pub filter_locked: bool,
}

/// https://developer.spotify.com/documentation/web-api/reference/#object-privateuserobject
#[derive(Debug, Deserialize)]
pub struct PrivateUser {
    pub country: String,
    pub display_name: String,
    pub email: String,
    pub explicit_content: ExplicitContentSettings,
    pub external_urls: ExternalUrl,
    pub followers: Option<Followers>,
    pub href: String,
    pub id: String,
    #[serde(default)]
    pub image: Vec<Image>,
    pub product: String,
    #[serde(default)]
    pub r#type: String,
    pub uri: String,
}

/// ?
#[derive(Debug, Deserialize)]
pub struct Tracks {
    pub tracks: Vec<Track>,
}
