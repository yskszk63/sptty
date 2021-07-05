use librespot::connect::spirc::Spirc;
use librespot::core::cache::Cache;
use librespot::core::config::ConnectConfig;
use librespot::core::config::DeviceType;
use librespot::core::config::VolumeCtrl;
use librespot::core::{authentication::Credentials, config::SessionConfig, session::Session};
use librespot::playback::config::Bitrate;
use librespot::playback::config::{AudioFormat, PlayerConfig};
use librespot::playback::player::Player;
use librespot::playback::{audio_backend, mixer};
use librespot::protocol::authentication::AuthenticationType;

pub async fn connect(token: &str) -> anyhow::Result<()> {
    let session_config = SessionConfig {
        ..Default::default()
    };
    let player_config = PlayerConfig {
        bitrate: Bitrate::Bitrate320,
        //normalisation: true,
        //passthrough: true,
        ..Default::default()
    };
    let connect_config = ConnectConfig {
        autoplay: false,
        device_type: DeviceType::Computer,
        name: "sptty".into(),
        volume: 0x5999, // 35%
        volume_ctrl: VolumeCtrl::Linear,
    };
    let mixer = mixer::find::<String>(None).unwrap();

    let credentials = Credentials {
        username: "".into(),
        auth_type: AuthenticationType::AUTHENTICATION_SPOTIFY_TOKEN,
        auth_data: token.into(),
    };
    let audio_cache = dirs::cache_dir().unwrap().join("sptty/audio");
    let cache = Cache::new(None, Some(audio_cache), None)?;

    let session = Session::connect(session_config, credentials, Some(cache)).await?;

    let sink_builder = || {
        let backend = audio_backend::find(Some("pulseaudio".into())).unwrap();
        let device = None;
        let format = AudioFormat::F32;
        (backend)(device, format)
    };
    let mixer = (mixer)(None);
    let (player, mut player_events) = Player::new(
        player_config,
        session.clone(),
        mixer.get_audio_filter(),
        sink_builder,
    );

    let (spric, spirc_task) = Spirc::new(connect_config, session.clone(), player, mixer);
    tokio::pin!(spirc_task);

    loop {
        tokio::select! {
            event = player_events.recv() => {
                if let Some(event) = event {
                    println!("{:?}", event);
                }
            }
            _ = &mut spirc_task => break
        }
    }
    drop(spric);

    Ok(())
}
