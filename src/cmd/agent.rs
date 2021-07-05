use std::env;
use std::path::Path;
use std::process::Stdio;

use tokio::fs;
use tokio::io;
use tokio::process::Command;

use super::*;

pub async fn run(env: &Environment) -> anyhow::Result<()> {
    let token = get_token(env, |url| println!("{}", url)).await?;
    connect(&token).await?;
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
