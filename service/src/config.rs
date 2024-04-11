use clap::builder::TypedValueParser as _;
use clap::Parser;
use log::LevelFilter;
use semver::{BuildMetadata, Prerelease, Version};
use serde::Deserialize;
use utoipa::IntoParams;

type APiVersionList = [&'static str; 1];

const DEFAULT_API_VERSION: &str = "0.0.1";
// Expand this array to include all valid API versions. Versions that have been
// completely removed should be removed from this list - they're no longer valid.
const API_VERSIONS: APiVersionList = [DEFAULT_API_VERSION];

static X_VERSION: &str = "x-version";

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Header)]
pub struct ApiVersion {
    /// The version of the API to use for a request.
    #[param(rename = "x-version", style = Simple, required, example = "0.0.1")]
    pub version: Version,
}

#[derive(Clone, Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Set the current semantic version of the endpoint API to expose to clients. All
    /// endpoints not contained in the specified version will not be exposed by the router.
    #[arg(short, long, env, default_value = DEFAULT_API_VERSION,
        value_parser = clap::builder::PossibleValuesParser::new(API_VERSIONS)
            .map(|s| s.parse::<String>().unwrap()),
        )]
    pub api_version: Option<String>,

    /// Sets the Postgresql database URI to connect to
    #[arg(
        short,
        long,
        env,
        default_value = "postgres://refactor:password@localhost:5432/refactor_platform"
    )]
    database_uri: Option<String>,

    /// The host interface to listen for incoming connections
    #[arg(short, long, default_value = "127.0.0.1")]
    pub interface: Option<String>,

    /// The host TCP port to listen for incoming connections
    #[arg(short, long, default_value_t = 4000)]
    pub port: u16,

    /// Set the log level verbosity threshold (level) to control what gets displayed on console output
    #[arg(
        short,
        long,
        default_value_t = LevelFilter::Warn,
        value_parser = clap::builder::PossibleValuesParser::new(["OFF", "ERROR", "WARN", "INFO", "DEBUG", "TRACE"])
            .map(|s| s.parse::<LevelFilter>().unwrap()),
        )]
    pub log_level_filter: LevelFilter,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        Config::parse()
    }

    pub fn api_version(&self) -> &str {
        self.api_version
            .as_ref()
            .expect("No API version string provided")
    }

    pub fn set_database_uri(mut self, database_uri: String) -> Self {
        self.database_uri = Some(database_uri);
        self
    }

    pub fn database_uri(&self) -> &str {
        self.database_uri
            .as_ref()
            .expect("No Database URI Provided")
    }
}

impl ApiVersion {
    pub fn new(version_str: &'static str) -> Self {
        ApiVersion {
            version: Version::parse(version_str).unwrap_or(Version {
                major: 0,
                minor: 0,
                patch: 1,
                pre: Prerelease::EMPTY,
                build: BuildMetadata::EMPTY,
            }),
        }
    }

    pub fn default_version() -> &'static str {
        DEFAULT_API_VERSION
    }

    pub fn field_name() -> &'static str {
        X_VERSION
    }

    pub fn versions() -> APiVersionList {
        API_VERSIONS
    }

    pub fn to_string(&self) -> String {
        self.version.to_string()
    }
}

impl Default for ApiVersion {
    fn default() -> Self {
        ApiVersion {
            version: Version::parse(DEFAULT_API_VERSION).unwrap_or(Version {
                major: 0,
                minor: 0,
                patch: 1,
                pre: Prerelease::EMPTY,
                build: BuildMetadata::EMPTY,
            }),
        }
    }
}
