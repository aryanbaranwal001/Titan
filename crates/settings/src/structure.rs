#![allow(deprecated)]

use crate::AppConfig;
use crate::cfg::SystemCallCfg;
use prost_types::Timestamp;
use proto_build::sf::ethereum::r#type::v2::{
    AccessTuple, AccountCreation, BalanceChange, BigInt, Block, BlockHeader,
    Call as TransactionTraceCall, Call, CodeChange, GasChange, Log, NonceChange,
    SetCodeAuthorization, StorageChange, TransactionReceipt, TransactionTrace, Uint64NestedArray,
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
    pub balance_changes: Option<Vec<ExtractedBalanceChange>>,
    pub code_changes: Option<Vec<ExtractedCodeChange>>,
    pub withdrawals: Option<Vec<ExtractedWithdrawal>>,
}

#[derive(Deserialize, Debug)]
pub struct ExtractedWithdrawal {
    pub index: Option<u64>,
    pub validator_index: Option<u64>,
    pub address: Option<Vec<u8>>,
    pub amount: Option<u64>,
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
