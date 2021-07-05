use super::*;

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
