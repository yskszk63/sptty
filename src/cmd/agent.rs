use std::env;
use std::path::Path;
use std::process::Stdio;

use tokio::fs;
use tokio::io;
use tokio::process::Command;
use tokio::sync::oneshot;

use super::*;

pub async fn run(env: &Environment) -> anyhow::Result<()> {
    let token = get_token(env, |url| println!("{}", url)).await?;
    let client = RestClient::new(env).await?;

    let devices = client
        .request::<_, model::Devices>("/v1/me/player/devices", Method::Get, Empty)
        .await?;
    let notify = if devices.devices.is_empty() {
        let (tx, rx) = oneshot::channel();
        tokio::spawn(async move {
            if let Ok(device_id) = rx.await {
                let req = model::TransferUserPlaybackRequest {
                    device_ids: vec![device_id],
                    play: false,
                };
                client
                    .request::<_, Empty>("/v1/me/player", Method::Put, req)
                    .await
                    .ok();
            }
        });
        Some(tx)
    } else {
        None
    };

    connect(&token, notify).await?;
    Ok(())
}

pub async fn start() -> anyhow::Result<()> {
    let status = Command::new("systemctl")
        .args(["start", "--user", "sptty"])
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .await?;
    if !status.success() {
        // TODO
        anyhow::bail!("failed to start connect. may be systemd user unit not installed.");
    }
    Ok(())
}

pub async fn kill() -> anyhow::Result<()> {
    let status = Command::new("systemctl")
        .args(["stop", "--user", "sptty"])
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .await?;
    if !status.success() {
        // TODO
        anyhow::bail!("failed to start connect. may be systemd user unit not installed.");
    }
    Ok(())
}

pub async fn install(force: bool) -> anyhow::Result<()> {
    async fn exists(path: &Path) -> io::Result<bool> {
        match fs::metadata(path).await {
            Ok(..) => Ok(true),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(false),
            Err(err) => Err(err),
        }
    }

    let userunit_home = Environment::systemd_user_dir();
    let service_file = userunit_home.join("sptty.service");
    if exists(&service_file).await? && !force {
        anyhow::bail!("{} already exists.", service_file.display())
    }

    let me = env::current_exe()?;
    let content = r#"[Unit]
Description=Lightweight Spotify daemon.

[Service]
ExecStart=@here agent run

[Install]
WantedBy=default.target
"#;
    let content = content.replace("@here", &me.display().to_string());

    fs::write(service_file, content).await?;
    Ok(())
}
