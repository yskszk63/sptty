use clap::{crate_authors, crate_description, crate_version, AppSettings, Clap, IntoApp};

mod auth;
mod cmd;
mod env;
pub(crate) mod model;
mod playback;
mod rest;

pub(crate) use auth::get_token;
pub(crate) use env::Environment;
pub(crate) use playback::connect;
pub(crate) use rest::{Empty, Method, RestClient, MayBeEmpty};

#[derive(Debug, Clap)]
#[clap(version = crate_version!(), author = crate_authors!(), about = crate_description!())]
#[clap(setting = AppSettings::InferSubcommands)]
struct Cli {
    /// Login to spotify.
    #[clap(long)]
    login: bool,

    #[clap(subcommand)]
    subcommand: Option<SubCommands>,
}

#[derive(Debug, Clap)]
#[clap(setting = AppSettings::InferSubcommands)]
enum SubCommands {
    /// Manage playback agent.
    #[clap(display_order = 0)]
    Agent(Agent),
    /// Manage connected spotify device.
    #[clap(display_order = 1)]
    Device(Device),
    /// List current playing playlist.
    #[clap(display_order = 2)]
    List,
    /// Skip next track current playing playlist.
    #[clap(display_order = 3)]
    NextTrack,
    /// Skip previous track current playing playlist.
    #[clap(display_order = 4)]
    PreviousTrack,
    /// Play track current playing playlist.
    #[clap(display_order = 5)]
    Play,
    /// Stop playing.
    #[clap(display_order = 6)]
    Stop,
    /// Open spotify client.
    #[clap(display_order = 7)]
    Open,
}

#[derive(Debug, Clap)]
#[clap(setting = AppSettings::InferSubcommands)]
struct Device {
    #[clap(subcommand)]
    subcommand: Option<DeviceSubCommands>,
}

#[derive(Debug, Clap)]
#[clap(setting = AppSettings::InferSubcommands)]
enum DeviceSubCommands {
    /// List connected devices.
    #[clap(display_order = 0)]
    List,
    /// Set active connection.
    #[clap(display_order = 1)]
    Set {
        /// Target device name prefix.
        name: String,
        /// Connect on play if ture.
        #[clap(long, short)]
        play: bool,

        /// specify by id
        #[clap(long, short)]
        id: bool,
    },
}

#[derive(Debug, Clap)]
#[clap(setting = AppSettings::InferSubcommands)]
struct Agent {
    #[clap(subcommand)]
    subcommand: Option<AgentSubCommands>,
}

#[derive(Debug, Clap)]
#[clap(setting = AppSettings::InferSubcommands)]
enum AgentSubCommands {
    /// Run playback agent on foreground. (default)
    #[clap(display_order = 0)]
    Run,
    /// Start playback anget on background. (as systemd user unit)
    #[clap(display_order = 1)]
    Start,
    /// Stop playback agent background process.
    #[clap(display_order = 2)]
    Kill,
    /// Install systemd user unit file.
    #[clap(display_order = 3)]
    Install {
        /// force install.
        #[clap(long, short)]
        force: bool,
    },
}

fn default_leading_authorization_url(url: String) {
    opener::open_browser(url).unwrap();
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let opts = Cli::parse();

    let env = Environment::load().await?;

    if opts.login {
        get_token(&env, default_leading_authorization_url).await?;
        return Ok(());
    }

    if opts.subcommand.is_none() {
        Cli::into_app().print_help()?;
        std::process::exit(-1);
    }

    let subcommand = opts.subcommand.unwrap();
    match subcommand {
        SubCommands::Agent(Agent {
            subcommand: None | Some(AgentSubCommands::Run),
        }) => cmd::agent::run(&env).await,

        SubCommands::Agent(Agent {
            subcommand: Some(AgentSubCommands::Start),
        }) => cmd::agent::start().await,

        SubCommands::Agent(Agent {
            subcommand: Some(AgentSubCommands::Kill),
        }) => cmd::agent::kill().await,

        SubCommands::Agent(Agent {
            subcommand: Some(AgentSubCommands::Install { force }),
        }) => cmd::agent::install(force).await,

        SubCommands::Device(Device {
            subcommand: None | Some(DeviceSubCommands::List),
        }) => cmd::device::list(&env).await,

        SubCommands::Device(Device {
            subcommand:
                Some(DeviceSubCommands::Set {
                    name,
                    play,
                    id: false,
                }),
        }) => cmd::device::set(&env, &name, play).await,

        SubCommands::Device(Device {
            subcommand:
                Some(DeviceSubCommands::Set {
                    name,
                    play,
                    id: true,
                }),
        }) => cmd::device::set_by_id(&env, &name, play).await,

        SubCommands::List => cmd::track::list(&env).await,
        SubCommands::Play => cmd::track::play(&env).await,
        SubCommands::Stop => cmd::track::stop(&env).await,
        SubCommands::NextTrack => cmd::track::next(&env).await,
        SubCommands::PreviousTrack => cmd::track::prev(&env).await,
        SubCommands::Open => cmd::open::open(),
        //_ => unimplemented!(),
    }
}
