#![allow(unused)]
use config::{Case, Config, ConfigError, Environment, File};
use proto_build::sf::ethereum::r#type::v2::Block;
use serde::Deserialize;

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

#[derive(Deserialize, Debug)]
pub struct ExtractedBlock {
    pub hash: Option<Vec<u8>>,
    pub number: Option<u64>,
    pub size: Option<u64>,
    pub detail_level: Option<DetailLevel>,
    pub ver: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub enum DetailLevel {
    Base,
    Extended,
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
        };
        Ok(extracted_block)
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct BlockCfg {
    pub hash: bool,
    pub number: bool,
    pub size: bool,
    pub detail_level: bool,
    pub ver: bool,
    // pub header: BlockHeader,
    // pub system_calls: Calls,
    // pub uncles: BlockHeader,
    // pub transaction_traces: TransactionTraces,
    // pub balance_changes: BalanceChanges,
    // pub code_changes: CodeChanges,
    // pub withdrawals: Withdrawals,
}

#[derive(Deserialize, Debug, Default)]
pub struct BlockHeader {
    pub enabled: bool,
    pub parent_hash: bool,
    pub uncle_hash: bool,
    pub coinbase: bool,
    pub state_root: bool,
    pub transactions_root: bool,
    pub receipt_root: bool,
    pub logs_bloom: bool,
    pub difficulty: bool,
    pub total_difficulty: bool,
    pub number: bool,
    pub gas_limit: bool,
    pub gas_used: bool,
    pub timestamp: bool,
    pub extra_data: bool,
    pub mix_hash: bool,
    pub nonce: bool,
    pub hash: bool,
    pub base_fee_per_gas: bool,
    pub withdrawals_root: bool,
    pub tx_dependency: bool,
    pub blob_gas_used: bool,
    pub excess_blob_gas: bool,
    pub parent_beacon_root: bool,
    pub requests_hash: bool,
}

#[derive(Deserialize, Debug, Default)]
pub struct TransactionTraces {
    pub enabled: bool,
    pub to: bool,
    pub nonce: bool,
    pub gas_price: bool,
    pub gas_limit: bool,
    pub value: bool,
    pub input: bool,
    pub v: bool,
    pub r: bool,
    pub s: bool,
    pub gas_used: bool,
    pub r#type: bool, // 'type' is a reserved keyword in Rust
    pub max_fee_per_gas: bool,
    pub max_priority_fee_per_gas: bool,
    pub index: bool,
    pub hash: bool,
    pub from: bool,
    pub return_data: bool,
    pub public_key: bool,
    pub begin_ordinal: bool,
    pub end_ordinal: bool,
    pub status: bool,
    pub blob_gas: bool,
    pub blob_gas_fee_cap: bool,
    pub blob_hashes: bool,
    pub receipt: Receipt,
    pub calls: Calls,
    pub set_code_authorizations: EnabledOnly,
}
#[derive(Deserialize, Debug, Default)]
pub struct Receipt {
    pub enabled: bool,
    pub state_root: bool,
    pub cumulative_gas_used: bool,
    pub logs_bloom: bool,
    pub blob_gas_used: bool,
    pub blob_gas_price: bool,
    pub logs: EnabledOnly,
}
#[derive(Deserialize, Debug, Default)]
pub struct Calls {
    pub enabled: bool,
    pub index: bool,
    pub parent_index: bool,
    pub depth: bool,
    pub call_type: bool,
    pub caller: bool,
    pub address: bool,
    pub address_delegates_to: bool,
    pub value: bool,
    pub gas_limit: bool,
    pub gas_consumed: bool,
    pub return_data: bool,
    pub input: bool,
    pub executed_code: bool,
    pub suicide: bool,
    pub keccak_preimages: bool,
    pub status_failed: bool,
    pub status_reverted: bool,
    pub failure_reason: bool,
    pub state_reverted: bool,
    pub begin_ordinal: bool,
    pub end_ordinal: bool,
    pub storage_changes: EnabledOnly,
    pub balance_changes: BalanceChanges,
    pub nonce_changes: EnabledOnly,
    pub gas_changes: EnabledOnly,
}
#[derive(Deserialize, Debug, Default)]
pub struct CodeChanges {
    pub enabled: bool,
    pub address: bool,
    pub old_hash: bool,
    pub old_code: bool,
    pub new_hash: bool,
    pub new_code: bool,
    pub ordinal: bool,
}
#[derive(Deserialize, Debug, Default)]
pub struct Withdrawals {
    pub enabled: bool,
    pub index: bool,
    pub validator_index: bool,
    pub address: bool,
    pub amount: bool,
}
#[derive(Deserialize, Debug, Default)]
pub struct EnabledOnly {
    pub enabled: bool,
}

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct BalanceChanges {
    pub enabled: bool,
    pub address: bool,
    pub old_value: bool,
    pub new_value: bool,
    pub reason: bool,
    pub ordinal: bool,
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
