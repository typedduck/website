use std::{
    env::{self, VarError},
    path::PathBuf,
};

use clap::{builder::NonEmptyStringValueParser, command, Arg, ArgAction, ArgMatches};
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use url::Url;

use crate::{CONFIG_ENV_FILE, CONFIG_ENV_PREFIX, CONFIG_FILE, CONFIG_PATHS};

/// The configuration for the site.
#[derive(Clone, Debug, Deserialize)]
pub struct Site {
    /// The title of the site.
    pub title: Box<str>,
    /// The language-code of the site.
    pub language: Box<str>,
    /// The base URL of the site.
    pub base: Url,
}

/// A source of assets to serve.
#[derive(Clone, Debug, Deserialize)]
pub struct AssetSource {
    /// The route to mount the assets on.
    pub route: Box<str>,
    /// The path to the assets on the filesystem. The CWD is that of the
    /// where the server is started. The paths may be relative or absolute and
    /// must be according to the filesystem.
    pub path: PathBuf,
}

/// Application settings.
#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    /// The host to bind the server to. This can be an IPv4 or IPv6 address or a
    /// domain name.
    pub host: Box<str>,
    /// The port to bind the server to.
    pub port: u16,
    /// The log filter directive to use as defined by the
    /// [`tracing_subscriber::EnvFilter`].
    pub log: Box<str>,
    /// The site configuration.
    pub site: Site,
    /// The assets to serve.
    #[serde(default)]
    pub assets: Box<[AssetSource]>,
}

impl Settings {
    /// Load the settings from the configuration file and environment variables.
    ///
    /// # Errors
    ///
    /// Returns an error if a configuration file could not be found or if there
    /// was an error parsing the configuration file.
    ///
    /// # Panics
    ///
    /// Panics if the configuration file path is not valid UTF-8. This should
    /// never happen as the path is hardcoded in the source code.
    pub fn load() -> Result<(Self, String), ConfigError> {
        let clargs = Self::clargs();
        let path = clargs.get_one::<String>("config");

        if let Some(path) = path.cloned() {
            Self::load_from_file(&path).map(|settings| (settings, path))
        } else {
            for path in CONFIG_PATHS {
                let path = PathBuf::from(path).join(CONFIG_FILE);
                if path.exists() {
                    let path = path.to_str().expect("valid UTF-8 path");
                    return Self::load_from_file(path).map(|settings| (settings, path.into()));
                }
            }

            let path = env::var(CONFIG_ENV_FILE).map_err(|err| match err {
                VarError::NotUnicode(_) => ConfigError::Message(format!(
                    "environnment variable `{CONFIG_ENV_FILE}` is not valid UTF-8"
                )),
                VarError::NotPresent => {
                    ConfigError::Message("no configuration file specified".to_string())
                }
            })?;

            Self::load_from_file(&path).map(|settings| (settings, path))
        }
    }

    /// Load the settings from a named configuration file. Missing values may be
    /// filled in by environment variables.
    ///
    /// # Errors
    ///
    /// Returns an error if a configuration file could not be found or if there
    /// was an error parsing the configuration file.
    pub fn load_from_file(path: impl AsRef<str>) -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name(path.as_ref()).required(true))
            .add_source(Environment::with_prefix(CONFIG_ENV_PREFIX).separator("_"))
            .set_default("port", 8080)?
            .set_default("host", "0.0.0.0")?
            .set_default("log", "error")?
            .set_default("site.language", "en")?
            .build()?;

        match config.try_deserialize() {
            Ok(settings) => {
                tracing::info!("loaded settings from {}", path.as_ref());
                Ok(settings)
            }
            Err(err) => Err(err),
        }
    }

    /// Parse the command line arguments.
    fn clargs() -> ArgMatches {
        let command = command!().arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_parser(NonEmptyStringValueParser::new())
                .action(ArgAction::Set)
                .env(CONFIG_ENV_FILE)
                .help("Path to a TOML file containing the configuration."),
        );

        command.get_matches()
    }
}
