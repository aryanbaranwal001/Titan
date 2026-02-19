#![allow(unused)]
use cfg::{BlockCfg, BlockHeaderCfg};
use config::{Case, Config, ConfigError, Environment, File};
use extract::{DetailLevel, ExtractedBlock, ExtractedBlockHeader};
use proto_build::sf::ethereum::r#type::v2::{BigInt, Block, BlockHeader, Uint64NestedArray};
use serde::{Deserialize, Serialize};
mod cfg;
mod extract;
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
        };
        Ok(extracted_block)
    }
    pub fn extract_header(&self, h: BlockHeader) -> ExtractedBlockHeader {
        // We use the header config section from your AppConfig
        let cfg = &self.block.blockheader;

        ExtractedBlockHeader {
            parent_hash: if cfg.parent_hash {
                Some(h.parent_hash)
            } else {
                None
            },
            uncle_hash: if cfg.uncle_hash {
                Some(h.uncle_hash)
            } else {
                None
            },
            coinbase: if cfg.coinbase { Some(h.coinbase) } else { None },
            state_root: if cfg.state_root {
                Some(h.state_root)
            } else {
                None
            },
            transactions_root: if cfg.transactions_root {
                Some(h.transactions_root)
            } else {
                None
            },
            receipt_root: if cfg.receipt_root {
                Some(h.receipt_root)
            } else {
                None
            },
            logs_bloom: if cfg.logs_bloom {
                Some(h.logs_bloom)
            } else {
                None
            },
            difficulty: if cfg.difficulty { h.difficulty } else { None },
            total_difficulty: if cfg.total_difficulty {
                h.total_difficulty
            } else {
                None
            },

            number: if cfg.number { Some(h.number) } else { None },
            gas_limit: if cfg.gas_limit {
                Some(h.gas_limit)
            } else {
                None
            },
            gas_used: if cfg.gas_used { Some(h.gas_used) } else { None },

            timestamp: if cfg.timestamp { h.timestamp } else { None },
            extra_data: if cfg.extra_data {
                Some(h.extra_data)
            } else {
                None
            },
            mix_hash: if cfg.mix_hash { Some(h.mix_hash) } else { None },
            nonce: if cfg.nonce { Some(h.nonce) } else { None },
            hash: if cfg.hash { Some(h.hash) } else { None },
            base_fee_per_gas: if cfg.base_fee_per_gas {
                h.base_fee_per_gas
            } else {
                None
            },

            withdrawals_root: if cfg.withdrawals_root {
                Some(h.withdrawals_root)
            } else {
                None
            },
            tx_dependency: if cfg.tx_dependency {
                h.tx_dependency
            } else {
                None
            },
            blob_gas_used: if cfg.blob_gas_used {
                h.blob_gas_used
            } else {
                None
            },
            excess_blob_gas: if cfg.excess_blob_gas {
                h.excess_blob_gas
            } else {
                None
            },
            parent_beacon_root: if cfg.parent_beacon_root {
                Some(h.parent_beacon_root)
            } else {
                None
            },
            requests_hash: if cfg.requests_hash {
                Some(h.requests_hash)
            } else {
                None
            },
        }
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
