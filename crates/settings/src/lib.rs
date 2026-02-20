#![allow(unused)]
use cfg::{BlockCfg, BlockHeaderCfg};
use config::{Case, Config, ConfigError, Environment, File};
use extract::{DetailLevel, ExtractedBlock, ExtractedBlockHeader, ExtractedSystemCall};
use proto_build::sf::ethereum::r#type::v2::{BigInt, Block, BlockHeader, Uint64NestedArray};
use serde::{Deserialize, Serialize};
pub mod cfg;
pub mod extract;
// NOTE: Whether the actual blockdata is a BlockHeader or Vec<BlockHeader>,
// the toggle structure used to select which fields to include
// remains the same. This toggle determines whether we fetch
// the full BlockHeader data or only specific fields.
//
// Examples include: block.header whose actual data is BlockHeader, and block.uncles whose
// actual data is Vec<BlockHeader>

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub auth_endpoint: String,
    pub endpoint_url: String,
    pub firehose_api_key: String,
    pub start_block: i64,
    pub end_block: u64,
    pub final_blocks_only: bool,
    // block configs
    pub block: BlockCfg,
}

pub type BoxBlockErr = Box<dyn std::error::Error + Send + Sync>;

impl AppConfig {
    pub async fn extract(&self, block: Block) -> Result<ExtractedBlock, BoxBlockErr> {
        let cfg = &self.block;
        let extracted_block = ExtractedBlock {
            hash: if cfg.hash { Some(block.hash) } else { None },
            number: if cfg.number { Some(block.number) } else { None },
            size: if cfg.size { Some(block.size) } else { None },
            detail_level: if cfg.detail_level {
                match block.detail_level {
                    0 => Some(DetailLevel::Extended),
                    1 => Some(DetailLevel::Base),
                    _ => Err("Invalid DetailLevel")?,
                }
            } else {
                None
            },
            ver: if cfg.size { Some(block.ver) } else { None },
            blockheader: if cfg.blockheader.enabled {
                block.header.map(|h| self.extract_header(h))
            } else {
                None
            },
            system_calls: if cfg.system_calls.enabled {
                let extracted: Vec<ExtractedSystemCall> = block
                    .system_calls
                    .into_iter()
                    .map(|c| self.extract_system_call(c, &cfg.system_calls))
                    .collect();

                if extracted.is_empty() {
                    None
                } else {
                    Some(extracted)
                }
            } else {
                None
            },
        };
        Ok(extracted_block)
    }
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

// my notes
//
// 1. check if every field implementation is correct i wrote with AI
// 2. check in detail how this serde remote works
