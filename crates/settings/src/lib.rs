use config::{Case, Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    firehose_api_key: String,
    block: BlockFlags,
}

#[derive(Deserialize, Debug)]
pub struct BlockFlags {
    start_block: u64,
    end_block: u64,
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(File::with_name("crates/settings/default.toml"))
            .add_source(File::with_name("config.toml"))
            .add_source(Environment::default().convert_case(Case::Snake))
            .build()?
            .try_deserialize()
    }
}
