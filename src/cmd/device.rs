use super::*;
use model::{Devices, TransferUserPlaybackRequest};

pub async fn list(env: &Environment) -> anyhow::Result<()> {
    let devices = RestClient::new(env)
        .await?
        .request::<_, Devices>("/v1/me/player/devices", Method::Get, Empty)
        .await?;
    for device in devices.devices {
        if device.is_active {
            println!("✔ {}", device.name);
        } else {
            println!("  {}", device.name);
        }
    }

    Ok(())
}

pub async fn set(env: &Environment, name: &str, play: bool) -> anyhow::Result<()> {
    let client = RestClient::new(env).await?;
    let devices = client
        .request::<_, Devices>("/v1/me/player/devices", Method::Get, Empty)
        .await?;

    let name = name.to_lowercase();
    let select = devices
        .devices
        .into_iter()
        .find(|device| device.name.to_lowercase().starts_with(&name));

    if let Some(select) = select {
        let req = TransferUserPlaybackRequest {
            device_ids: vec![select.id.clone()],
            play,
        };
        client
            .request::<_, Empty>("/v1/me/player", Method::Put, req)
            .await?;
    } else {
        anyhow::bail!("no match device found.");
    }
    Ok(())
}
