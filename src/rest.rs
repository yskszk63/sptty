use hyper::header::CONTENT_LENGTH;
use reqwest::{Client, Url};
use serde::{de::DeserializeOwned, Serialize};

use super::get_token;

pub enum Method {
    Get,
    Post,
    Put,
}

pub trait Input {
    fn to_json(&self) -> anyhow::Result<Vec<u8>>;
}

pub trait Output: Sized {
    fn from_json(b: &[u8]) -> anyhow::Result<Self>;
}

#[derive(Debug)]
pub struct Empty;

impl Input for Empty {
    fn to_json(&self) -> anyhow::Result<Vec<u8>> {
        Ok(vec![])
    }
}

impl Output for Empty {
    fn from_json(_: &[u8]) -> anyhow::Result<Self> {
        Ok(Self)
    }
}

impl<T> Input for T
where
    T: Serialize,
{
    fn to_json(&self) -> anyhow::Result<Vec<u8>> {
        let r = serde_json::to_vec(self)?;
        Ok(r)
    }
}

impl<T> Output for T
where
    T: DeserializeOwned,
{
    fn from_json(b: &[u8]) -> anyhow::Result<Self> {
        let r = serde_json::from_slice(b)?;
        Ok(r)
    }
}

#[derive(Debug)]
pub enum MayBeEmpty<T> {
    Present(T),
    Empty,
}

impl<T> Output for MayBeEmpty<T>
where
    T: DeserializeOwned,
{
    fn from_json(b: &[u8]) -> anyhow::Result<Self> {
        if b.is_empty() {
            return Ok(Self::Empty);
        }
        let r = serde_json::from_slice(b)?;
        Ok(Self::Present(r))
    }
}

pub struct RestClient {
    base: Url,
    token: String,
    client: Client,
}

impl RestClient {
    pub async fn new(env: &super::Environment) -> anyhow::Result<Self> {
        let base = Url::parse(&env.api_endpoint)?;
        let token = get_token(env, super::default_leading_authorization_url).await?;

        let client = Client::new();
        Ok(Self {
            base,
            token,
            client,
        })
    }

    pub async fn request<I, O>(&self, path: &str, method: Method, req: I) -> anyhow::Result<O>
    where
        I: Input,
        O: Output,
    {
        let url = self.base.join(path)?;

        let body = req.to_json()?;
        log::debug!("request: {}", String::from_utf8_lossy(&body));
        let req = match method {
            Method::Get => self.client.get(url),
            Method::Post => self.client.post(url),
            Method::Put => self.client.put(url),
        };
        let res = req
            .bearer_auth(&self.token)
            .header(CONTENT_LENGTH, body.len())
            .body(body)
            .send()
            .await?;

        if !res.status().is_success() {
            anyhow::bail!(
                "failed to request({}).: {}",
                res.status().to_string(),
                res.text().await?,
            );
        }

        let body = res.bytes().await?;
        log::debug!("response: {}", String::from_utf8_lossy(&body));
        Ok(O::from_json(&body)?)
    }
}
