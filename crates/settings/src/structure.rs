#![allow(deprecated)]

use crate::AppConfig;
use crate::cfg::SystemCallCfg;
use prost_types::Timestamp;
use proto_build::sf::ethereum::r#type::v2::{
    AccessTuple, Call as TransactionTraceCall, Log, SetCodeAuthorization, TransactionReceipt,
    TransactionTrace,
};
use proto_build::sf::ethereum::r#type::v2::{
    AccountCreation, BalanceChange, BigInt, Block, BlockHeader, Call, CodeChange, GasChange,
    NonceChange, StorageChange, Uint64NestedArray,
};
use serde::Deserialize;
use std::{collections::HashMap, fmt};

pub mod extract;
pub mod format;

// this is incomplete
#[derive(Deserialize, Debug)]
pub struct ExtractedBlock {
    pub hash: Option<Vec<u8>>,
    pub number: Option<u64>,
    pub size: Option<u64>,
    pub detail_level: Option<DetailLevel>,
    pub ver: Option<i32>,
    pub blockheader: Option<ExtractedBlockHeader>,
    pub system_calls: Option<Vec<ExtractedSystemCall>>,
    pub uncles: Option<Vec<ExtractedUncleBlockHeader>>,
    pub transaction_traces: Option<Vec<ExtractedTransactionTraces>>,
}

#[derive(Deserialize, Debug)]
pub struct ExtractedBlockHeader {
    pub parent_hash: Option<Vec<u8>>,
    pub uncle_hash: Option<Vec<u8>>,
    pub coinbase: Option<Vec<u8>>,
    pub state_root: Option<Vec<u8>>,
    pub transactions_root: Option<Vec<u8>>,
    pub receipt_root: Option<Vec<u8>>,
    pub logs_bloom: Option<Vec<u8>>,
    pub difficulty: Option<BigInt>,
    pub total_difficulty: Option<BigInt>,
    pub number: Option<u64>,
    pub gas_limit: Option<u64>,
    pub gas_used: Option<u64>,

    #[serde(with = "option_timestamp")]
    pub timestamp: Option<Timestamp>,
    pub extra_data: Option<Vec<u8>>,
    pub mix_hash: Option<Vec<u8>>,
    pub nonce: Option<u64>,
    pub hash: Option<Vec<u8>>,
    pub base_fee_per_gas: Option<BigInt>,
    pub withdrawals_root: Option<Vec<u8>>,
    pub tx_dependency: Option<Uint64NestedArray>,
    pub blob_gas_used: Option<u64>,
    pub excess_blob_gas: Option<u64>,
    pub parent_beacon_root: Option<Vec<u8>>,
    pub requests_hash: Option<Vec<u8>>,
}

#[derive(Deserialize, Debug)]
pub struct ExtractedTransactionTraces {
    pub to: Option<Vec<u8>>,
    pub nonce: Option<u64>,
    pub gas_price: Option<BigInt>,
    pub gas_limit: Option<u64>,
    pub value: Option<BigInt>,
    pub input: Option<Vec<u8>>,
    pub v: Option<Vec<u8>>,
    pub r: Option<Vec<u8>>,
    pub s: Option<Vec<u8>>,
    pub gas_used: Option<u64>,
    pub r#type: Option<i32>,
    pub max_fee_per_gas: Option<BigInt>,
    pub max_priority_fee_per_gas: Option<BigInt>,
    pub index: Option<u32>,
    pub hash: Option<Vec<u8>>,
    pub from: Option<Vec<u8>>,
    pub return_data: Option<Vec<u8>>,
    pub public_key: Option<Vec<u8>>,
    pub begin_ordinal: Option<u64>,
    pub end_ordinal: Option<u64>,
    pub status: Option<i32>,
    pub blob_gas: Option<u64>,
    pub blob_gas_fee_cap: Option<BigInt>,
    pub blob_hashes: Option<Vec<Vec<u8>>>,

    pub access_list: Option<Vec<ExtractedAccessTuple>>,
    pub receipt: Option<ExtractedTransactionReceipt>,
    pub calls: Option<Vec<ExtractedTracsactionTraceCall>>,
    pub set_code_authorizations: Option<Vec<ExtractedSetCodeAuthorization>>,
}

#[derive(Deserialize, Debug)]
pub struct ExtractedSetCodeAuthorization {
    pub discarded: Option<bool>,
    pub chain_id: Option<Vec<u8>>,
    pub address: Option<Vec<u8>>,
    pub nonce: Option<u64>,
    pub v: Option<u32>,
    pub r: Option<Vec<u8>>,
    pub s: Option<Vec<u8>>,
    pub authority: Option<Vec<u8>>,
}

#[derive(Deserialize, Debug)]
pub struct ExtractedTracsactionTraceCall {
    pub index: Option<u32>,
    pub parent_index: Option<u32>,
    pub depth: Option<u32>,
    pub call_type: Option<i32>,
    pub caller: Option<Vec<u8>>,
    pub address: Option<Vec<u8>>,
    pub address_delegates_to: Option<Vec<u8>>,
    pub value: Option<BigInt>,
    pub gas_limit: Option<u64>,
    pub gas_consumed: Option<u64>,
    pub return_data: Option<Vec<u8>>,
    pub input: Option<Vec<u8>>,
    pub executed_code: Option<bool>,
    pub suicide: Option<bool>,
    pub keccak_preimages: Option<HashMap<String, String>>,
    pub status_failed: Option<bool>,
    pub status_reverted: Option<bool>,
    pub failure_reason: Option<String>,
    pub state_reverted: Option<bool>,
    pub begin_ordinal: Option<u64>,
    pub end_ordinal: Option<u64>,
    pub storage_changes: Option<Vec<ExtractedStorageChange>>,
    pub balance_changes: Option<Vec<ExtractedBalanceChange>>,
    pub nonce_changes: Option<Vec<ExtractedNonceChange>>,
    pub code_changes: Option<Vec<ExtractedCodeChange>>,
    pub gas_changes: Option<Vec<ExtractedGasChange>>,
    pub account_creations: Option<Vec<ExtractedAccountCreations>>,
}

#[derive(Deserialize, Debug, Default)]
pub struct ExtractedTransactionReceipt {
    pub state_root: Option<Vec<u8>>,
    pub cumulative_gas_used: Option<u64>,
    pub logs_bloom: Option<Vec<u8>>,
    pub logs: Option<Vec<ExtractedLog>>,
    pub blob_gas_used: Option<u64>,
    pub blob_gas_price: Option<BigInt>,
}

#[derive(Deserialize, Debug, Default)]
pub struct ExtractedLog {
    pub address: Option<Vec<u8>>,
    pub topics: Option<Vec<Vec<u8>>>,
    pub data: Option<Vec<u8>>,
    pub index: Option<u32>,
    pub block_index: Option<u32>,
    pub ordinal: Option<u64>,
}

#[derive(Deserialize, Debug, Default)]
pub struct ExtractedAccessTuple {
    pub address: Option<Vec<u8>>,
    pub storage_keys: Option<Vec<Vec<u8>>>,
}

#[derive(Deserialize, Debug)]
pub struct ExtractedUncleBlockHeader {
    pub parent_hash: Option<Vec<u8>>,
    pub uncle_hash: Option<Vec<u8>>,
    pub coinbase: Option<Vec<u8>>,
    pub state_root: Option<Vec<u8>>,
    pub transactions_root: Option<Vec<u8>>,
    pub receipt_root: Option<Vec<u8>>,
    pub logs_bloom: Option<Vec<u8>>,
    pub difficulty: Option<BigInt>,
    pub total_difficulty: Option<BigInt>,
    pub number: Option<u64>,
    pub gas_limit: Option<u64>,
    pub gas_used: Option<u64>,

    #[serde(with = "option_timestamp")]
    pub timestamp: Option<Timestamp>,
    pub extra_data: Option<Vec<u8>>,
    pub mix_hash: Option<Vec<u8>>,
    pub nonce: Option<u64>,
    pub hash: Option<Vec<u8>>,
    pub base_fee_per_gas: Option<BigInt>,
    pub withdrawals_root: Option<Vec<u8>>,
    pub tx_dependency: Option<Uint64NestedArray>,
    pub blob_gas_used: Option<u64>,
    pub excess_blob_gas: Option<u64>,
    pub parent_beacon_root: Option<Vec<u8>>,
    pub requests_hash: Option<Vec<u8>>,
}

#[derive(Deserialize, Debug)]
pub struct ExtractedSystemCall {
    pub index: Option<u32>,
    pub parent_index: Option<u32>,
    pub depth: Option<u32>,
    pub call_type: Option<i32>,
    pub caller: Option<Vec<u8>>,
    pub address: Option<Vec<u8>>,
    pub address_delegates_to: Option<Vec<u8>>,
    pub value: Option<BigInt>,
    pub gas_limit: Option<u64>,
    pub gas_consumed: Option<u64>,
    pub return_data: Option<Vec<u8>>,
    pub input: Option<Vec<u8>>,
    pub executed_code: Option<bool>,
    pub suicide: Option<bool>,
    pub keccak_preimages: Option<HashMap<String, String>>,
    pub status_failed: Option<bool>,
    pub status_reverted: Option<bool>,
    pub failure_reason: Option<String>,
    pub state_reverted: Option<bool>,
    pub begin_ordinal: Option<u64>,
    pub end_ordinal: Option<u64>,
    pub storage_changes: Option<Vec<ExtractedStorageChange>>,
    pub balance_changes: Option<Vec<ExtractedBalanceChange>>,
    pub nonce_changes: Option<Vec<ExtractedNonceChange>>,
    pub code_changes: Option<Vec<ExtractedCodeChange>>,
    pub gas_changes: Option<Vec<ExtractedGasChange>>,
    pub account_creations: Option<Vec<ExtractedAccountCreations>>,
}

#[derive(Deserialize, Debug)]
pub struct ExtractedCodeChange {
    pub address: Option<Vec<u8>>,
    pub old_hash: Option<Vec<u8>>,
    pub old_code: Option<Vec<u8>>,
    pub new_hash: Option<Vec<u8>>,
    pub new_code: Option<Vec<u8>>,
    pub ordinal: u64,
}

#[derive(Deserialize, Debug)]
pub struct ExtractedGasChange {
    pub old_value: Option<u64>,
    pub new_value: Option<u64>,
    pub reason: Option<i32>,
    pub ordinal: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct ExtractedAccountCreations {
    pub account: Option<Vec<u8>>,
    pub ordinal: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct ExtractedStorageChange {
    pub address: Option<Vec<u8>>,
    pub key: Option<Vec<u8>>,
    pub old_value: Option<Vec<u8>>,
    pub new_value: Option<Vec<u8>>,
    pub ordinal: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct ExtractedBalanceChange {
    pub address: Option<Vec<u8>>,
    pub old_value: Option<BigInt>,
    pub new_value: Option<BigInt>,
    pub reason: Option<i32>,
    pub ordinal: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct ExtractedNonceChange {
    pub address: Option<Vec<u8>>,
    pub old_value: Option<BigInt>,
    pub new_value: Option<BigInt>,
    pub ordinal: Option<u64>,
}

impl fmt::Display for ExtractedBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Block {{")?;
        if let Some(ref v) = self.hash {
            writeln!(f, "  hash: {:?},", v)?;
        }
        if let Some(ref v) = self.number {
            writeln!(f, "  number: {},", v)?;
        }
        if let Some(ref v) = self.size {
            writeln!(f, "  size: {},", v)?;
        }
        if let Some(ref v) = self.detail_level {
            writeln!(f, "  detail_level: {:?},", v)?;
        }
        if let Some(ref v) = self.ver {
            writeln!(f, "  ver: {},", v)?;
        }

        if let Some(ref header) = self.blockheader {
            let header_str = format!("{}", header).replace('\n', "\n  ");
            writeln!(f, "  blockheader: {}", header_str)?;
        }

        if let Some(ref calls) = self.system_calls {
            writeln!(f, "  system_calls: [")?;
            for call in calls {
                let system_call_str = format!("  {}", call).replace('\n', "\n  ");
                writeln!(f, "{},", system_call_str)?;
            }
            writeln!(f, "  ],")?;
        }

        if let Some(ref headers) = self.uncles {
            writeln!(f, "  uncles: [")?;
            for header in headers {
                let header_str = format!("  {}", header).replace('\n', "\n  ");
                writeln!(f, "{},", header_str)?;
            }
            writeln!(f, "  ],")?;
        }

        if let Some(ref traces) = self.transaction_traces {
            writeln!(f, "  transaction_traces: [")?;
            for trace in traces {
                let trace_str = format!("  {}", trace).replace('\n', "\n  ");
                writeln!(f, "{},", trace_str)?;
            }
            writeln!(f, "  ],")?;
        }

        write!(f, "}}")
    }
}

impl fmt::Display for ExtractedTransactionTraces {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "TransactionTrace {{")?;

        macro_rules! display_field {
            ($field_name:expr, $value:expr) => {
                if let Some(ref v) = $value {
                    writeln!(f, "  {}: {:?},", $field_name, v)?;
                }
            };
        }

        display_field!("to", self.to);
        display_field!("nonce", self.nonce);
        display_field!("gas_price", self.gas_price);
        display_field!("gas_limit", self.gas_limit);
        display_field!("value", self.value);
        display_field!("input", self.input);
        display_field!("v", self.v);
        display_field!("r", self.r);
        display_field!("s", self.s);
        display_field!("gas_used", self.gas_used);
        display_field!("type", self.r#type);
        display_field!("max_fee_per_gas", self.max_fee_per_gas);
        display_field!("max_priority_fee_per_gas", self.max_priority_fee_per_gas);
        display_field!("index", self.index);
        display_field!("hash", self.hash);
        display_field!("from", self.from);
        display_field!("return_data", self.return_data);
        display_field!("public_key", self.public_key);
        display_field!("begin_ordinal", self.begin_ordinal);
        display_field!("end_ordinal", self.end_ordinal);
        display_field!("status", self.status);

        display_field!("blob_gas", self.blob_gas);
        display_field!("blob_gas_fee_cap", self.blob_gas_fee_cap);
        display_field!("blob_hashes", self.blob_hashes);

        if let Some(ref v) = self.access_list {
            writeln!(f, "  access_list: [")?;
            for item in v {
                let str = format!("      {}", item).replace('\n', "\n      ");
                writeln!(f, "{},", str)?;
            }
            writeln!(f, "  ],")?;
        }
        if let Some(ref v) = self.receipt {
            let str = format!("  {}", v).replace('\n', "\n  ");
            writeln!(f, "  receipt: {},", str)?;
        }
        if let Some(ref v) = self.calls {
            writeln!(f, "  calls: [")?;
            for item in v {
                let str = format!("      {}", item).replace('\n', "\n      ");
                writeln!(f, "{},", str)?;
            }
            writeln!(f, "  ],")?;
        }
        if let Some(ref v) = self.set_code_authorizations {
            writeln!(f, "  set_code_authorizations: [")?;
            for item in v {
                let str = format!("      {}", item).replace('\n', "\n      ");
                writeln!(f, "{},", str)?;
            }
            writeln!(f, "  ],")?;
        }

        write!(f, "}}")
    }
}

impl fmt::Display for ExtractedAccessTuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "AccessTuple {{")?;
        if let Some(ref v) = self.address {
            writeln!(f, "  address: {:?},", v)?;
        }
        if let Some(ref v) = self.storage_keys {
            writeln!(f, "  storage_keys: {:?},", v)?;
        }
        write!(f, "}}")
    }
}

impl fmt::Display for ExtractedTransactionReceipt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "TransactionReceipt {{")?;
        macro_rules! display_field {
            ($field_name:expr, $value:expr) => {
                if let Some(ref v) = $value {
                    writeln!(f, "  {}: {:?},", $field_name, v)?;
                }
            };
        }

        display_field!("state_root", self.state_root);
        display_field!("cumulative_gas_used", self.cumulative_gas_used);
        display_field!("logs_bloom", self.logs_bloom);

        if let Some(ref v) = self.logs {
            writeln!(f, "  logs: [")?;
            for item in v {
                let str = format!("      {}", item).replace('\n', "\n      ");
                writeln!(f, "{},", str)?;
            }
            writeln!(f, "  ],")?;
        }

        display_field!("blob_gas_used", self.blob_gas_used);
        display_field!("blob_gas_price", self.blob_gas_price);

        write!(f, "}}")
    }
}

impl fmt::Display for ExtractedLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Log {{")?;
        macro_rules! display_field {
            ($field_name:expr, $value:expr) => {
                if let Some(ref v) = $value {
                    writeln!(f, "  {}: {:?},", $field_name, v)?;
                }
            };
        }

        display_field!("address", self.address);
        display_field!("topics", self.topics);
        display_field!("data", self.data);
        display_field!("index", self.index);
        display_field!("block_index", self.block_index);
        display_field!("ordinal", self.ordinal);

        write!(f, "}}")
    }
}

impl fmt::Display for ExtractedTracsactionTraceCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "TransactionTraceCall {{")?;

        macro_rules! display_field {
            ($field_name:expr, $value:expr) => {
                if let Some(ref v) = $value {
                    writeln!(f, "  {}: {:?},", $field_name, v)?;
                }
            };
        }

        display_field!("index", self.index);
        display_field!("parent_index", self.parent_index);
        display_field!("depth", self.depth);
        display_field!("call_type", self.call_type);
        display_field!("caller", self.caller);
        display_field!("address", self.address);
        display_field!("address_delegates_to", self.address_delegates_to);
        display_field!("value", self.value);
        display_field!("gas_limit", self.gas_limit);
        display_field!("gas_consumed", self.gas_consumed);
        display_field!("return_data", self.return_data);
        display_field!("input", self.input);
        display_field!("executed_code", self.executed_code);
        display_field!("suicide", self.suicide);
        display_field!("keccak_preimages", self.keccak_preimages);
        display_field!("status_failed", self.status_failed);
        display_field!("status_reverted", self.status_reverted);
        display_field!("failure_reason", self.failure_reason);
        display_field!("state_reverted", self.state_reverted);
        display_field!("begin_ordinal", self.begin_ordinal);
        display_field!("end_ordinal", self.end_ordinal);

        if let Some(ref v) = self.storage_changes {
            writeln!(f, "  storage_changes: [")?;
            for item in v {
                let str = format!("      {}", item).replace('\n', "\n      ");
                writeln!(f, "{},", str)?;
            }
            writeln!(f, "  ],")?;
        }
        if let Some(ref v) = self.balance_changes {
            writeln!(f, "  balance_changes: [")?;
            for item in v {
                let str = format!("      {}", item).replace('\n', "\n      ");
                writeln!(f, "{},", str)?;
            }
            writeln!(f, "  ],")?;
        }
        if let Some(ref v) = self.nonce_changes {
            writeln!(f, "  nonce_changes: [")?;
            for item in v {
                let str = format!("      {}", item).replace('\n', "\n      ");
                writeln!(f, "{},", str)?;
            }
            writeln!(f, "  ],")?;
        }
        if let Some(ref v) = self.code_changes {
            writeln!(f, "  code_changes: [")?;
            for item in v {
                let str = format!("      {}", item).replace('\n', "\n      ");
                writeln!(f, "{},", str)?;
            }
            writeln!(f, "  ],")?;
        }
        if let Some(ref v) = self.gas_changes {
            writeln!(f, "  gas_changes: [")?;
            for item in v {
                let str = format!("      {}", item).replace('\n', "\n      ");
                writeln!(f, "{},", str)?;
            }
            writeln!(f, "  ],")?;
        }
        if let Some(ref v) = self.account_creations {
            writeln!(f, "  account_creations: [")?;
            for item in v {
                let str = format!("      {}", item).replace('\n', "\n      ");
                writeln!(f, "{},", str)?;
            }
            writeln!(f, "  ],")?;
        }

        write!(f, "}}")
    }
}

impl fmt::Display for ExtractedSetCodeAuthorization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "SetCodeAuthorization {{")?;
        macro_rules! display_field {
            ($field_name:expr, $value:expr) => {
                if let Some(ref v) = $value {
                    writeln!(f, "  {}: {:?},", $field_name, v)?;
                }
            };
        }

        display_field!("discarded", self.discarded);
        display_field!("chain_id", self.chain_id);
        display_field!("address", self.address);
        display_field!("nonce", self.nonce);
        display_field!("v", self.v);
        display_field!("r", self.r);
        display_field!("s", self.s);
        display_field!("authority", self.authority);

        write!(f, "}}")
    }
}

impl fmt::Display for ExtractedBlockHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Header {{")?;

        macro_rules! display_field {
            ($field_name:expr, $value:expr) => {
                if let Some(ref v) = $value {
                    writeln!(f, "  {}: {:?},", $field_name, v)?;
                }
            };
        }

        display_field!("parent_hash", self.parent_hash);
        display_field!("uncle_hash", self.uncle_hash);
        display_field!("coinbase", self.coinbase);
        display_field!("state_root", self.state_root);
        display_field!("transactions_root", self.transactions_root);
        display_field!("receipt_root", self.receipt_root);
        display_field!("logs_bloom", self.logs_bloom);
        display_field!("difficulty", self.difficulty);
        display_field!("total_difficulty", self.total_difficulty);
        display_field!("number", self.number);
        display_field!("gas_limit", self.gas_limit);
        display_field!("gas_used", self.gas_used);
        display_field!("timestamp", self.timestamp);
        display_field!("extra_data", self.extra_data);
        display_field!("mix_hash", self.mix_hash);
        display_field!("nonce", self.nonce);
        display_field!("hash", self.hash);
        display_field!("base_fee_per_gas", self.base_fee_per_gas);
        display_field!("withdrawals_root", self.withdrawals_root);
        display_field!("tx_dependency", self.tx_dependency);
        display_field!("blob_gas_used", self.blob_gas_used);
        display_field!("excess_blob_gas", self.excess_blob_gas);
        display_field!("parent_beacon_root", self.parent_beacon_root);
        display_field!("requests_hash", self.requests_hash);

        write!(f, "}}")
    }
}

impl fmt::Display for ExtractedUncleBlockHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Header {{")?;

        macro_rules! display_field {
            ($field_name:expr, $value:expr) => {
                if let Some(ref v) = $value {
                    writeln!(f, "  {}: {:?},", $field_name, v)?;
                }
            };
        }

        display_field!("parent_hash", self.parent_hash);
        display_field!("uncle_hash", self.uncle_hash);
        display_field!("coinbase", self.coinbase);
        display_field!("state_root", self.state_root);
        display_field!("transactions_root", self.transactions_root);
        display_field!("receipt_root", self.receipt_root);
        display_field!("logs_bloom", self.logs_bloom);
        display_field!("difficulty", self.difficulty);
        display_field!("total_difficulty", self.total_difficulty);
        display_field!("number", self.number);
        display_field!("gas_limit", self.gas_limit);
        display_field!("gas_used", self.gas_used);
        display_field!("timestamp", self.timestamp);
        display_field!("extra_data", self.extra_data);
        display_field!("mix_hash", self.mix_hash);
        display_field!("nonce", self.nonce);
        display_field!("hash", self.hash);
        display_field!("base_fee_per_gas", self.base_fee_per_gas);
        display_field!("withdrawals_root", self.withdrawals_root);
        display_field!("tx_dependency", self.tx_dependency);
        display_field!("blob_gas_used", self.blob_gas_used);
        display_field!("excess_blob_gas", self.excess_blob_gas);
        display_field!("parent_beacon_root", self.parent_beacon_root);
        display_field!("requests_hash", self.requests_hash);

        write!(f, "}}")
    }
}

impl fmt::Display for ExtractedSystemCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  SystemCall {{")?;

        if let Some(v) = self.index {
            writeln!(f, "    index: {},", v)?;
        }
        if let Some(v) = self.parent_index {
            writeln!(f, "    parent_index: {},", v)?;
        }
        if let Some(v) = self.depth {
            writeln!(f, "    depth: {},", v)?;
        }
        if let Some(v) = self.call_type {
            writeln!(f, "    call_type: {},", v)?;
        }

        // Byte arrays formatted as Hex
        if let Some(ref v) = self.caller {
            writeln!(f, "    caller: {:?},", v)?;
        }
        if let Some(ref v) = self.address {
            writeln!(f, "    address: {:?},", v)?;
        }
        if let Some(ref v) = self.address_delegates_to {
            writeln!(f, "    delegates_to: {:?},", v)?;
        }

        if let Some(ref v) = self.value {
            writeln!(f, "    value: {:?},", v)?;
        } // BigInt uses Debug format
        if let Some(v) = self.gas_limit {
            writeln!(f, "    gas_limit: {},", v)?;
        }
        if let Some(v) = self.gas_consumed {
            writeln!(f, "    gas_consumed: {},", v)?;
        }

        if let Some(ref v) = self.return_data {
            writeln!(f, "    return_data: {:?},", v)?;
        }
        if let Some(ref v) = self.input {
            writeln!(f, "    input: {:?},", v)?;
        }

        if let Some(v) = self.executed_code {
            writeln!(f, "    executed_code: {},", v)?;
        }
        if let Some(v) = self.suicide {
            writeln!(f, "    suicide: {},", v)?;
        }

        if let Some(ref v) = self.keccak_preimages {
            writeln!(f, "    keccak_preimages: {} entries,", v.len())?;
        }

        if let Some(v) = self.status_failed {
            writeln!(f, "    status_failed: {},", v)?;
        }
        if let Some(v) = self.status_reverted {
            writeln!(f, "    status_reverted: {},", v)?;
        }

        if let Some(ref v) = self.failure_reason {
            writeln!(f, "    failure_reason: \"{}\",", v)?;
        }

        if let Some(v) = self.state_reverted {
            writeln!(f, "    state_reverted: {},", v)?;
        }
        if let Some(v) = self.begin_ordinal {
            writeln!(f, "    begin_ordinal: {},", v)?;
        }
        if let Some(v) = self.end_ordinal {
            writeln!(f, "    end_ordinal: {},", v)?;
        }

        if let Some(ref v) = self.storage_changes {
            writeln!(f, "    storage_changes: [")?;
            for item in v {
                let str = format!("      {}", item).replace('\n', "\n      ");
                writeln!(f, "{},", str)?;
            }
            writeln!(f, "    ],")?;
        }
        if let Some(ref v) = self.balance_changes {
            writeln!(f, "    balance_changes: [")?;
            for item in v {
                let str = format!("      {}", item).replace('\n', "\n      ");
                writeln!(f, "{},", str)?;
            }
            writeln!(f, "    ],")?;
        }
        if let Some(ref v) = self.nonce_changes {
            writeln!(f, "    nonce_changes: [")?;
            for item in v {
                let str = format!("      {}", item).replace('\n', "\n      ");
                writeln!(f, "{},", str)?;
            }
            writeln!(f, "    ],")?;
        }
        if let Some(ref v) = self.code_changes {
            writeln!(f, "    code_changes: [")?;
            for item in v {
                let str = format!("      {}", item).replace('\n', "\n      ");
                writeln!(f, "{},", str)?;
            }
            writeln!(f, "    ],")?;
        }
        if let Some(ref v) = self.gas_changes {
            writeln!(f, "    gas_changes: [")?;
            for item in v {
                let str = format!("      {}", item).replace('\n', "\n      ");
                writeln!(f, "{},", str)?;
            }
            writeln!(f, "    ],")?;
        }
        if let Some(ref v) = self.account_creations {
            writeln!(f, "    account_creations: [")?;
            for item in v {
                let str = format!("      {}", item).replace('\n', "\n      ");
                writeln!(f, "{},", str)?;
            }
            writeln!(f, "    ],")?;
        }

        write!(f, "  }}")
    }
}

impl fmt::Display for ExtractedStorageChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "StorageChange {{")?;
        if let Some(ref v) = self.address {
            writeln!(f, "  address: {:?},", v)?;
        }
        if let Some(ref v) = self.key {
            writeln!(f, "  key: {:?},", v)?;
        }
        if let Some(ref v) = self.old_value {
            writeln!(f, "  old_value: {:?},", v)?;
        }
        if let Some(ref v) = self.new_value {
            writeln!(f, "  new_value: {:?},", v)?;
        }
        if let Some(v) = self.ordinal {
            writeln!(f, "  ordinal: {},", v)?;
        }
        write!(f, "}}")
    }
}

impl fmt::Display for ExtractedBalanceChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "BalanceChange {{")?;
        if let Some(ref v) = self.address {
            writeln!(f, "  address: {:?},", v)?;
        }
        if let Some(ref v) = self.old_value {
            writeln!(f, "  old_value: {:?},", v)?;
        }
        if let Some(ref v) = self.new_value {
            writeln!(f, "  new_value: {:?},", v)?;
        }
        if let Some(v) = self.reason {
            writeln!(f, "  reason: {},", v)?;
        }
        if let Some(v) = self.ordinal {
            writeln!(f, "  ordinal: {},", v)?;
        }
        write!(f, "}}")
    }
}

impl fmt::Display for ExtractedNonceChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "NonceChange {{")?;
        if let Some(ref v) = self.address {
            writeln!(f, "  address: {:?},", v)?;
        }
        if let Some(ref v) = self.old_value {
            writeln!(f, "  old_value: {:?},", v)?;
        }
        if let Some(ref v) = self.new_value {
            writeln!(f, "  new_value: {:?},", v)?;
        }
        if let Some(v) = self.ordinal {
            writeln!(f, "  ordinal: {},", v)?;
        }
        write!(f, "}}")
    }
}

impl fmt::Display for ExtractedCodeChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "CodeChange {{")?;
        if let Some(ref v) = self.address {
            writeln!(f, "  address: {:?},", v)?;
        }
        if let Some(ref v) = self.old_hash {
            writeln!(f, "  old_hash: {:?},", v)?;
        }
        if let Some(ref v) = self.old_code {
            writeln!(f, "  old_code: {:?},", v)?;
        }
        if let Some(ref v) = self.new_hash {
            writeln!(f, "  new_hash: {:?},", v)?;
        }
        if let Some(ref v) = self.new_code {
            writeln!(f, "  new_code: {:?},", v)?;
        }
        writeln!(f, "  ordinal: {},", self.ordinal)?;
        write!(f, "}}")
    }
}

impl fmt::Display for ExtractedGasChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "GasChange {{")?;
        if let Some(v) = self.old_value {
            writeln!(f, "  old_value: {},", v)?;
        }
        if let Some(v) = self.new_value {
            writeln!(f, "  new_value: {},", v)?;
        }
        if let Some(v) = self.reason {
            writeln!(f, "  reason: {},", v)?;
        }
        if let Some(v) = self.ordinal {
            writeln!(f, "  ordinal: {},", v)?;
        }
        write!(f, "}}")
    }
}

impl fmt::Display for ExtractedAccountCreations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "AccountCreation {{")?;
        if let Some(ref v) = self.account {
            writeln!(f, "  account: {:?},", v)?;
        }
        if let Some(v) = self.ordinal {
            writeln!(f, "  ordinal: {},", v)?;
        }
        write!(f, "}}")
    }
}

#[derive(Deserialize)]
#[serde(remote = "prost_types::Timestamp")]
pub struct TimestampDef {
    pub seconds: i64,
    pub nanos: i32,
}

// task: replace this boilderplate code with serde_with trait implementation
pub mod option_timestamp {
    use super::TimestampDef;
    // use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use serde::{Deserialize, Deserializer};

    // pub fn serialize<S>(
    //     value: &Option<prost_types::Timestamp>,
    //     serializer: S,
    // ) -> Result<S::Ok, S::Error>
    // where
    //     S: Serializer,
    // {
    //     #[derive(Serialize)]
    //     struct Helper<'a>(#[serde(with = "TimestampDef")] &'a prost_types::Timestamp);
    //
    //     value.as_ref().map(Helper).serialize(serializer)
    // }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<prost_types::Timestamp>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper(#[serde(with = "TimestampDef")] prost_types::Timestamp);

        let helper = Option::<Helper>::deserialize(deserializer)?;
        Ok(helper.map(|h| h.0))
    }
}

#[derive(Deserialize, Debug)]
pub enum DetailLevel {
    Base,
    Extended,
}

//why the fuck are we even using Deserialize
