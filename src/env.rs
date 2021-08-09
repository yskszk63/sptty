use std::env;
use std::path::PathBuf;

use crate::auth::AuthorizationConfig;

const API_ENDPOINT: &str = "https://api.spotify.com";

#[derive(Debug)]
pub struct Environment {
    pub auth_config: AuthorizationConfig,
    pub api_endpoint: String,
}

impl Environment {
    pub async fn load() -> anyhow::Result<Self> {
        let auth_config = AuthorizationConfig::load(&Self::config_file()).await?;
        Ok(Self {
            auth_config,
            api_endpoint: API_ENDPOINT.into(),
        })
    }

    pub fn config_dir() -> PathBuf {
        if let Ok(env) = env::var("SPTTY_CONIG_DIR") {
            return PathBuf::from(env);
        }
        dirs::config_dir().unwrap().join("sptty/")
    }

    pub fn config_file() -> PathBuf {
        Self::config_dir().join("config.toml")
    }

    pub fn systemd_user_runtime_dir() -> PathBuf {
        let dir = dirs::runtime_dir().expect("no $XDG_DATA_HOME found.");
        dir.join("systemd/user")
    }
}
