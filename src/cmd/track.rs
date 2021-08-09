use super::*;

pub async fn list(env: &Environment) -> anyhow::Result<()> {
    let client = RestClient::new(env).await?;
    let current_playing = client
        .request::<_, MayBeEmpty<model::CurrentlyPlayingContext>>(
            "/v1/me/player",
            Method::Get,
            Empty,
        )
        .await?;
    let current_playing = if let MayBeEmpty::Present(p) = current_playing {
        p
    } else {
        anyhow::bail!("currently not playing.");
    };
    if let Some(ctx) = &current_playing.context {
        log::debug!("{:?}", ctx);
        match ctx.r#type.as_ref() {
            "artist" => {
                let user = client
                    .request::<_, model::PrivateUser>("/v1/me", Method::Get, Empty)
                    .await?;
                let url = format!(
                    "{}/top-tracks?additional_types=track&market={}",
                    &ctx.href, &user.country
                );
                let tracks = client
                    .request::<_, model::Tracks>(&url, Method::Get, Empty)
                    .await?;
                for track in tracks.tracks {
                    println!(
                        "{} {} ({})",
                        track.uri,
                        track.name,
                        track
                            .artists
                            .iter()
                            .map(|a| a.name.clone())
                            .collect::<Vec<_>>()
                            .join(" ")
                    );
                }
            }
            "playlist" => {
                let url = format!("{}/tracks?additional_types=track", &ctx.href);
                let playlist = client
                    .request::<_, model::Paging<model::PlaylistTrack>>(&url, Method::Get, Empty)
                    .await?;
                for track in playlist.items {
                    match track.track {
                        Some(model::TrackOrEpisode::Track { inner }) => {
                            println!(
                                "{} {} ({})",
                                inner.uri,
                                inner.name,
                                inner
                                    .artists
                                    .iter()
                                    .map(|a| a.name.clone())
                                    .collect::<Vec<_>>()
                                    .join(" ")
                            );
                        }
                        Some(model::TrackOrEpisode::Episode { .. }) => {
                            todo!();
                        }
                        None => todo!(),
                    }
                }
            }
            "album" => {
                let url = format!("{}/tracks", &ctx.href);
                let tracks = client
                    .request::<_, model::Paging<model::Track>>(&url, Method::Get, Empty)
                    .await?;
                for track in tracks.items {
                    println!(
                        "{} {} ({})",
                        track.uri,
                        track.name,
                        track
                            .artists
                            .iter()
                            .map(|a| a.name.clone())
                            .collect::<Vec<_>>()
                            .join(" ")
                    );
                }
            }
            "show" => todo!(),
            s => anyhow::bail!("unknown context: {}", s),
        }
    } else {
        anyhow::bail!("could not list tracks. because currently playing is not artist, playlist, album, show.");
    };
    Ok(())
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

pub async fn play(env: &Environment, uri: &str) -> anyhow::Result<()> {
    let request = model::StartResumeAUsersPlaybackRequest {
        uris: Some(vec![uri.into()]),
        ..Default::default()
    };
    RestClient::new(env)
        .await?
        .request::<_, Empty>("/v1/me/player/play", Method::Put, request)
        .await?;
    Ok(())
}

pub async fn resume(env: &Environment) -> anyhow::Result<()> {
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
