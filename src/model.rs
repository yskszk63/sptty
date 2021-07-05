use serde::{Deserialize, Serialize};

/// https://developer.spotify.com/documentation/web-api/reference/#object-deviceobject
#[derive(Debug, Deserialize)]
pub struct Device {
    pub id: String,
    pub is_active: bool,
    pub is_private_session: bool,
    pub is_restricted: bool,
    pub name: String,
    pub r#type: String,
    pub volume_percent: u8,
}

/// https://developer.spotify.com/documentation/web-api/reference/#object-devicesobject
#[derive(Debug, Deserialize)]
pub struct Devices {
    pub devices: Vec<Device>,
}

#[derive(Debug, Serialize)]
pub struct TransferUserPlaybackRequest {
    pub device_ids: Vec<String>,
    pub play: bool,
}
