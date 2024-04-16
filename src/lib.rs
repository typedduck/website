#![allow(clippy::multiple_crate_versions)]

/// Name of the default configuration file.
pub const CONFIG_FILE: &str = "website.toml";
/// Default paths to search for the configuration file if not specified by the
/// user either through a command-line argument or environment variable. The
/// paths are appended with [`CONFIG_FILE`] to form the full path to the
/// configuration file. Paths are searched in order, and the first file found is
/// used.
pub const CONFIG_PATHS: [&str; 1] = ["."];
/// Name of the environment variable prefix.
pub const CONFIG_ENV_PREFIX: &str = "WEBSITE";
/// Name of the environment variable for the configuration file. This is used
/// to override the default configuration file. It is ignored if the user
/// specifies a configuration file through a command-line argument.
pub const CONFIG_ENV_FILE: &str = "WEBSITE_CONFIG";

pub mod handler;
pub mod settings;

mod state;

pub use state::AppState;

mod template;
pub use template::HtmlTemplate;
