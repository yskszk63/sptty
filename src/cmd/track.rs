use serde::{Deserialize, Serialize};

use super::*;

/// https://developer.spotify.com/documentation/web-api/reference/#object-deviceobject
#[derive(Debug, Deserialize)]
struct Device {
    id: String,
    is_active: bool,
    is_private_session: bool,
    is_restricted: bool,
    name: String,
    r#type: String,
    volume_percent: u8,
}

/// https://developer.spotify.com/documentation/web-api/reference/#object-devicesobject
#[derive(Debug, Deserialize)]
struct Devices {
    devices: Vec<Device>,
}

#[derive(Debug, Serialize)]
struct TransferUserPlaybackRequest {
    device_ids: Vec<String>,
    play: bool,
}

pub async fn next(env: &Environment) -> anyhow::Result<()> {
    RestClient::new(env)
        .await?
        .request::<_, Empty>("/v1/me/player/next", Method::Post, Empty)
        .await?;
    Ok(())
}

pub async fn prev(env: &Environment) -> anyhow::Result<()> {
    RestClient::new(env)
        .await?
        .request::<_, Empty>("/v1/me/player/previous", Method::Post, Empty)
        .await?;
    Ok(())
}

pub async fn play(env: &Environment) -> anyhow::Result<()> {
    RestClient::new(env)
        .await?
        .request::<_, Empty>("/v1/me/player/play", Method::Put, Empty)
        .await?;
    Ok(())
}

pub async fn stop(env: &Environment) -> anyhow::Result<()> {
    RestClient::new(env)
        .await?
        .request::<_, Empty>("/v1/me/player/pause", Method::Put, Empty)
        .await?;
    Ok(())
}
