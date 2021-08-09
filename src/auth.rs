use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use hyper::{server::conn::Http, service::service_fn, Body, Response};
use rand::distributions::Distribution;
use reqwest::{Client, Url};
use ring::digest;
use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio::net::TcpListener;
use tokio::sync::oneshot;
use tokio::sync::Mutex;

const DEFAULT_AUTHORIZATION_ENDPOINT: &str = "https://accounts.spotify.com/authorize";
const DEFAULT_TOKEN_ENDPOINT: &str = "https://accounts.spotify.com/api/token";

#[derive(Debug, Deserialize)]
pub struct AuthorizationConfig {
    pub client_id: String,
    pub redirect_uri: String,
    #[serde(default = "AuthorizationConfig::default_authorization_endpoint")]
    pub authorization_endpoint: String,
    #[serde(default = "AuthorizationConfig::default_token_endpoint")]
    pub token_endpoint: String,
}

impl AuthorizationConfig {
    pub async fn load<P>(path: P) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        let buf = fs::read(path).await?;
        let result = toml::from_slice(&buf)?;
        Ok(result)
    }

    fn default_authorization_endpoint() -> String {
        DEFAULT_AUTHORIZATION_ENDPOINT.into()
    }

    fn default_token_endpoint() -> String {
        DEFAULT_TOKEN_ENDPOINT.into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AccessToken {
    access_token: String,
    token_type: String,
    scope: String,
    expires_in: u64,
    refresh_token: String,
}

#[derive(Debug)]
struct PkceCodeVerifierChars;

impl Distribution<u8> for PkceCodeVerifierChars {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        const RANGE: u32 = 26 + 26 + 10 + 4;
        const GEN_ASCII_STR_CHARSET: &[u8] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_.-~";

        // https://github.com/rust-random/rand/blob/067238f05753d18c7cc032717bef03a7f5244a8c/src/distributions/other.rs#L105
        loop {
            let var = rng.next_u32() >> (32 - 7); // 128bit
            if var < RANGE {
                return GEN_ASCII_STR_CHARSET[var as usize];
            }
        }
    }
}

fn gen_code_verifier() -> String {
    let mut rand = rand::thread_rng();
    PkceCodeVerifierChars
        .sample_iter(&mut rand)
        .take(128)
        .map(char::from)
        .collect::<String>()
}

fn code_challenge(code_verifier: &str) -> String {
    let hash = digest::digest(&digest::SHA256, code_verifier.as_bytes());
    base64::encode_config(&hash, base64::URL_SAFE_NO_PAD)
}

fn authorization_url(
    config: &AuthorizationConfig,
    code_challenge: &str,
) -> anyhow::Result<(Url, String)> {
    let state = rand::random::<[u8; 16]>();
    let state = base64::encode_config(&state, base64::URL_SAFE_NO_PAD);

    let mut url = Url::parse(&config.authorization_endpoint)?;
    url.query_pairs_mut()
        .append_pair("client_id", &config.client_id)
        .append_pair("response_type", "code")
        .append_pair("redirect_uri", &config.redirect_uri)
        .append_pair("code_challenge_method", "S256")
        .append_pair("code_challenge", code_challenge)
        .append_pair(
            "scope",
            "streaming user-read-email user-read-private user-read-playback-state user-top-read",
        )
        .append_pair("state", &state);

    Ok((url, state))
}

async fn get_authorization_code<F>(
    config: &AuthorizationConfig,
    code_verifier: &str,
    mut url_callback: F,
) -> anyhow::Result<String>
where
    F: FnMut(String),
{
    let challenge = code_challenge(code_verifier);
    let (url, csrf_token) = authorization_url(config, &challenge)?;
    (url_callback)(url.to_string());

    let redirect_uri = Url::parse(&config.redirect_uri)?;

    let listener = TcpListener::bind((
        redirect_uri.host().unwrap().to_string(),
        redirect_uri.port().unwrap_or(80),
    ))
    .await?;

    let (tx, rx) = oneshot::channel();
    let tx = Arc::new(Mutex::new(Some(tx)));
    let accept = async {
        let (sock, _) = listener.accept().await?;
        let tx = tx.clone();
        let csrf_token = csrf_token.clone();
        let func = service_fn(move |req| {
            let tx = tx.clone();
            let csrf_token = csrf_token.clone();
            async move {
                let uri = req.uri();
                let url = Url::parse("http://example.com/")?.join(&uri.to_string())?;

                let query = url.query_pairs().collect::<HashMap<_, _>>();

                if let (Some(code), Some(state)) = (query.get("code"), query.get("state")) {
                    if let Some(tx) = tx.lock().await.take() {
                        if state != &csrf_token {
                            anyhow::bail!("state mismatch.");
                        }
                        tx.send(code.to_string()).ok();
                        let res = Response::builder().body(Body::from("ok"))?;
                        return anyhow::Result::<_>::Ok(res);
                    }
                }

                let res = Response::builder().status(204).body(Body::empty())?;
                anyhow::Result::<_>::Ok(res)
            }
        });

        Http::new().serve_connection(sock, func).await?;
        anyhow::Result::<_>::Ok(())
    };

    tokio::select! {
        r = accept => {
            r?;
            anyhow::bail!("failed to serve code.") // may be unreachable
        }
        code = rx => Ok(code?),
    }
}

#[derive(Debug, Serialize)]
struct AuthorizationFlowRequest {
    client_id: String,
    grant_type: String,
    code: String,
    redirect_uri: String,
    code_verifier: String,
}

#[derive(Debug, Serialize)]
struct RefreshTokenFlowRequest {
    grant_type: String,
    refresh_token: String,
    client_id: String,
}

async fn get_token_from_code(
    config: &AuthorizationConfig,
    code_verifier: &str,
    code: &str,
) -> anyhow::Result<AccessToken> {
    let form = AuthorizationFlowRequest {
        client_id: config.client_id.clone(),
        grant_type: "authorization_code".to_string(),
        code: code.to_string(),
        redirect_uri: config.redirect_uri.clone(),
        code_verifier: code_verifier.to_string(),
    };

    let response = Client::new()
        .post(&config.token_endpoint)
        .form(&form)
        .send()
        .await?;
    if !response.status().is_success() {
        anyhow::bail!("failed to get access token. {}", response.text().await?,);
    }

    let result = response.bytes().await?;
    let token = serde_json::from_slice(&result)?;
    Ok(token)
}

async fn refresh_token(
    config: &AuthorizationConfig,
    token: &AccessToken,
) -> anyhow::Result<AccessToken> {
    let form = RefreshTokenFlowRequest {
        grant_type: "refresh_token".to_string(),
        refresh_token: token.refresh_token.clone(),
        client_id: config.client_id.clone(),
    };

    let response = Client::new()
        .post(&config.token_endpoint)
        .form(&form)
        .send()
        .await?;
    if !response.status().is_success() {
        anyhow::bail!("failed to get access token. {}", response.text().await?,);
    }

    let result = response.bytes().await?;
    let token = serde_json::from_slice(&result)?;
    Ok(token)
}

pub async fn authenticate<F>(env: &super::Environment, url_callback: F) -> anyhow::Result<()>
where
    F: FnMut(String),
{
    let cache_path = dirs::cache_dir().unwrap().join("sptty/token");
    fs::create_dir_all(cache_path.parent().unwrap()).await?;

    let verifier = gen_code_verifier();
    let code = get_authorization_code(&env.auth_config, &verifier, url_callback).await?;
    let token = get_token_from_code(&env.auth_config, &verifier, &code).await?;

    let json = serde_json::to_string(&token)?;
    fs::write(&cache_path, json).await?;

    Ok(())
}

pub async fn get_token<F>(env: &super::Environment, url_callback: F) -> anyhow::Result<String>
where
    F: FnMut(String),
{
    let cache_path = dirs::cache_dir().unwrap().join("sptty/token");
    fs::create_dir_all(cache_path.parent().unwrap()).await?;

    if let Ok(cache) = fs::read_to_string(&cache_path).await {
        let token = serde_json::from_str::<AccessToken>(&cache)?;
        let token = refresh_token(&env.auth_config, &token).await?;
        let json = serde_json::to_string(&token)?;
        fs::write(&cache_path, json).await?;
        return Ok(token.access_token);
    }

    let verifier = gen_code_verifier();
    let code = get_authorization_code(&env.auth_config, &verifier, url_callback).await?;
    let token = get_token_from_code(&env.auth_config, &verifier, &code).await?;

    let json = serde_json::to_string(&token)?;
    fs::write(&cache_path, json).await?;

    Ok(token.access_token)
}
