#![allow(unused)]
use config::{Case, Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub auth_endpoint: String,
    pub endpoint_url: String,
    pub firehose_api_key: String,
    pub block: BlockFlags,
}

#[derive(Deserialize, Debug)]
pub struct BlockFlags {
    pub start_block: i64,
    pub end_block: u64,
    pub final_blocks_only: bool,
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
