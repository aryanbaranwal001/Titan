#![allow(deprecated)]

use crate::AppConfig;
use crate::cfg::SystemCallCfg;
use prost_types::Timestamp;
use proto_build::sf::ethereum::r#type::v2::{
    AccessTuple, Call as TransactionTraceCall,
    Log, SetCodeAuthorization, TransactionReceipt,
    TransactionTrace,
};
use proto_build::sf::ethereum::r#type::v2::{
    AccountCreation, BalanceChange, BigInt, Block, BlockHeader, Call, CodeChange, GasChange,
    NonceChange, StorageChange, Uint64NestedArray,
};
use serde::Deserialize;
use std::{collections::HashMap, fmt};

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

//why the fuck are we even using Deserialize
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

impl AppConfig {
    pub fn extract_header(&self, h: BlockHeader) -> ExtractedBlockHeader {
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

    pub fn extract_uncles(&self, h: BlockHeader) -> ExtractedUncleBlockHeader {
        let cfg = &self.block.blockheader;

        ExtractedUncleBlockHeader {
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

    pub fn extract_system_call(&self, c: Call, cfg: &SystemCallCfg) -> ExtractedSystemCall {
        let cfg = &self.block.system_calls;

        ExtractedSystemCall {
            index: if cfg.index { Some(c.index) } else { None },
            parent_index: if cfg.parent_index {
                Some(c.parent_index)
            } else {
                None
            },
            depth: if cfg.depth { Some(c.depth) } else { None },
            call_type: if cfg.call_type {
                Some(c.call_type)
            } else {
                None
            },
            caller: if cfg.caller { Some(c.caller) } else { None },
            address: if cfg.address { Some(c.address) } else { None },
            address_delegates_to: if cfg.address_delegates_to {
                c.address_delegates_to
            } else {
                None
            },
            value: if cfg.value { c.value } else { None },
            gas_limit: if cfg.gas_limit {
                Some(c.gas_limit)
            } else {
                None
            },
            gas_consumed: if cfg.gas_consumed {
                Some(c.gas_consumed)
            } else {
                None
            },
            return_data: if cfg.return_data {
                Some(c.return_data)
            } else {
                None
            },
            input: if cfg.input { Some(c.input) } else { None },
            executed_code: if cfg.executed_code {
                Some(c.executed_code)
            } else {
                None
            },
            suicide: if cfg.suicide { Some(c.suicide) } else { None },
            keccak_preimages: if cfg.keccak_preimages {
                Some(c.keccak_preimages)
            } else {
                None
            },
            status_failed: if cfg.status_failed {
                Some(c.status_failed)
            } else {
                None
            },
            status_reverted: if cfg.status_reverted {
                Some(c.status_reverted)
            } else {
                None
            },
            failure_reason: if cfg.failure_reason {
                Some(c.failure_reason)
            } else {
                None
            },
            state_reverted: if cfg.state_reverted {
                Some(c.state_reverted)
            } else {
                None
            },
            begin_ordinal: if cfg.begin_ordinal {
                Some(c.begin_ordinal)
            } else {
                None
            },
            end_ordinal: if cfg.end_ordinal {
                Some(c.end_ordinal)
            } else {
                None
            },
            storage_changes: Some(
                c.storage_changes
                    .into_iter()
                    .map(|s| self.extract_storage_change(s))
                    .collect(),
            ),
            balance_changes: Some(
                c.balance_changes
                    .into_iter()
                    .map(|b| self.extract_balance_change(b))
                    .collect(),
            ),
            code_changes: Some(
                c.code_changes
                    .into_iter()
                    .map(|cc| self.extract_code_change(cc))
                    .collect(),
            ),
            nonce_changes: Some(
                c.nonce_changes
                    .into_iter()
                    .map(|n| self.extract_nonce_change(n))
                    .collect(),
            ),
            gas_changes: Some(
                c.gas_changes
                    .into_iter()
                    .map(|g| self.extract_gas_change(g))
                    .collect(),
            ),
            account_creations: Some(
                c.account_creations
                    .into_iter()
                    .map(|a| self.extract_account_creation(a))
                    .collect(),
            ),
        }
    }

    fn extract_storage_change(&self, s: StorageChange) -> ExtractedStorageChange {
        let cfg = &self.block.system_calls.storage_changes;
        ExtractedStorageChange {
            address: if cfg.address { Some(s.address) } else { None },
            key: if cfg.key { Some(s.key) } else { None },
            old_value: if cfg.old_value {
                Some(s.old_value)
            } else {
                None
            },
            new_value: if cfg.new_value {
                Some(s.new_value)
            } else {
                None
            },
            ordinal: if cfg.ordinal { Some(s.ordinal) } else { None },
        }
    }

    fn extract_balance_change(&self, b: BalanceChange) -> ExtractedBalanceChange {
        let cfg = &self.block.system_calls.balance_changes;
        ExtractedBalanceChange {
            address: if cfg.address { Some(b.address) } else { None },
            old_value: if cfg.old_value { b.old_value } else { None },
            new_value: if cfg.new_value { b.new_value } else { None },
            reason: if cfg.reason { Some(b.reason) } else { None },
            ordinal: if cfg.ordinal { Some(b.ordinal) } else { None },
        }
    }

    fn extract_nonce_change(&self, n: NonceChange) -> ExtractedNonceChange {
        let cfg = &self.block.system_calls.nonce_changes;
        ExtractedNonceChange {
            address: if cfg.address { Some(n.address) } else { None },
            old_value: if cfg.old_value {
                Some(BigInt {
                    bytes: n.old_value.to_be_bytes().to_vec(),
                })
            } else {
                None
            },
            new_value: if cfg.new_value {
                Some(BigInt {
                    bytes: n.new_value.to_be_bytes().to_vec(),
                })
            } else {
                None
            },
            ordinal: if cfg.ordinal { Some(n.ordinal) } else { None },
        }
    }

    fn extract_code_change(&self, cc: CodeChange) -> ExtractedCodeChange {
        let cfg = &self.block.system_calls.code_changes;
        ExtractedCodeChange {
            address: if cfg.address { Some(cc.address) } else { None },
            old_hash: if cfg.old_hash {
                Some(cc.old_hash)
            } else {
                None
            },
            old_code: if cfg.old_code {
                Some(cc.old_code)
            } else {
                None
            },
            new_hash: if cfg.new_hash {
                Some(cc.new_hash)
            } else {
                None
            },
            new_code: if cfg.new_code {
                Some(cc.new_code)
            } else {
                None
            },
            ordinal: cc.ordinal,
        }
    }

    fn extract_gas_change(&self, g: GasChange) -> ExtractedGasChange {
        let cfg = &self.block.system_calls.gas_changes;
        ExtractedGasChange {
            old_value: if cfg.old_value {
                Some(g.old_value)
            } else {
                None
            },
            new_value: if cfg.new_value {
                Some(g.new_value)
            } else {
                None
            },
            reason: if cfg.reason { Some(g.reason) } else { None },
            ordinal: if cfg.ordinal { Some(g.ordinal) } else { None },
        }
    }

    fn extract_account_creation(&self, a: AccountCreation) -> ExtractedAccountCreations {
        let cfg = &self.block.system_calls.account_creations;
        ExtractedAccountCreations {
            account: if cfg.account { Some(a.account) } else { None },
            ordinal: if cfg.ordinal { Some(a.ordinal) } else { None },
        }
    }

    pub fn extract_transaction_traces(
        &self,
        t: TransactionTrace,
    ) -> ExtractedTransactionTraces {
        let cfg = &self.block.transaction_traces;

        ExtractedTransactionTraces {
            to: if cfg.to { Some(t.to) } else { None },
            nonce: if cfg.nonce { Some(t.nonce) } else { None },
            gas_price: if cfg.gas_price { t.gas_price } else { None },
            gas_limit: if cfg.gas_limit { Some(t.gas_limit) } else { None },
            value: if cfg.value { t.value } else { None },
            input: if cfg.input { Some(t.input) } else { None },
            v: if cfg.v { Some(t.v) } else { None },
            r: if cfg.r { Some(t.r) } else { None },
            s: if cfg.s { Some(t.s) } else { None },
            gas_used: if cfg.gas_used { Some(t.gas_used) } else { None },
            r#type: if cfg.r#type { Some(t.r#type) } else { None },
            max_fee_per_gas: if cfg.max_fee_per_gas {
                t.max_fee_per_gas
            } else {
                None
            },
            max_priority_fee_per_gas: if cfg.max_priority_fee_per_gas {
                t.max_priority_fee_per_gas
            } else {
                None
            },
            index: if cfg.index { Some(t.index) } else { None },
            hash: if cfg.hash { Some(t.hash) } else { None },
            from: if cfg.from { Some(t.from) } else { None },
            return_data: if cfg.return_data { Some(t.return_data) } else { None },
            public_key: if cfg.public_key { Some(t.public_key) } else { None },
            begin_ordinal: if cfg.begin_ordinal {
                Some(t.begin_ordinal)
            } else {
                None
            },
            end_ordinal: if cfg.end_ordinal { Some(t.end_ordinal) } else { None },
            status: if cfg.status { Some(t.status) } else { None },
            blob_gas: if cfg.blob_gas { t.blob_gas } else { None },
            blob_gas_fee_cap: if cfg.blob_gas_fee_cap {
                t.blob_gas_fee_cap
            } else {
                None
            },
            blob_hashes: if cfg.blob_hashes { Some(t.blob_hashes) } else { None },

            access_list: Some(
                t.access_list
                    .into_iter()
                    .map(|a| self.extract_access_tuple(a))
                    .collect(),
            ),
            receipt: t.receipt.map(|r| self.extract_transaction_receipt(r)),
            calls: Some(
                t.calls
                    .into_iter()
                    .map(|c| self.extract_transaction_trace_call(c))
                    .collect(),
            ),
            set_code_authorizations: Some(
                t.set_code_authorizations
                    .into_iter()
                    .map(|s| self.extract_set_code_authorization(s))
                    .collect(),
            ),
        }
    }

    fn extract_access_tuple(&self, a: AccessTuple) -> ExtractedAccessTuple {
        let cfg = &self.block.transaction_traces.access_list;
        ExtractedAccessTuple {
            address: if cfg.address { Some(a.address) } else { None },
            storage_keys: if cfg.storage_keys {
                Some(a.storage_keys)
            } else {
                None
            },
        }
    }

    fn extract_transaction_receipt(&self, r: TransactionReceipt) -> ExtractedTransactionReceipt {
        let cfg = &self.block.transaction_traces.receipt;
        ExtractedTransactionReceipt {
            state_root: if cfg.state_root {
                Some(r.state_root)
            } else {
                None
            },
            cumulative_gas_used: if cfg.cumulative_gas_used {
                Some(r.cumulative_gas_used)
            } else {
                None
            },
            logs_bloom: if cfg.logs_bloom {
                Some(r.logs_bloom)
            } else {
                None
            },
            logs: Some(
                r.logs
                    .into_iter()
                    .map(|l| self.extract_log(l))
                    .collect(),
            ),
            blob_gas_used: if cfg.blob_gas_used {
                r.blob_gas_used
            } else {
                None
            },
            blob_gas_price: if cfg.blob_gas_price {
                r.blob_gas_price
            } else {
                None
            },
        }
    }

    fn extract_log(&self, l: Log) -> ExtractedLog {
        let cfg = &self.block.transaction_traces.receipt.logs;
        ExtractedLog {
            address: if cfg.address { Some(l.address) } else { None },
            topics: if cfg.topics { Some(l.topics) } else { None },
            data: if cfg.data { Some(l.data) } else { None },
            index: if cfg.index { Some(l.index) } else { None },
            block_index: if cfg.block_index {
                Some(l.block_index)
            } else {
                None
            },
            ordinal: if cfg.ordinal { Some(l.ordinal) } else { None },
        }
    }

    fn extract_transaction_trace_call(
        &self,
        c: TransactionTraceCall,
    ) -> ExtractedTracsactionTraceCall {
        let cfg = &self.block.transaction_traces.calls;

        ExtractedTracsactionTraceCall {
            index: if cfg.index { Some(c.index) } else { None },
            parent_index: if cfg.parent_index {
                Some(c.parent_index)
            } else {
                None
            },
            depth: if cfg.depth { Some(c.depth) } else { None },
            call_type: if cfg.call_type {
                Some(c.call_type)
            } else {
                None
            },
            caller: if cfg.caller { Some(c.caller) } else { None },
            address: if cfg.address { Some(c.address) } else { None },
            address_delegates_to: if cfg.address_delegates_to {
                c.address_delegates_to
            } else {
                None
            },
            value: if cfg.value { c.value } else { None },
            gas_limit: if cfg.gas_limit {
                Some(c.gas_limit)
            } else {
                None
            },
            gas_consumed: if cfg.gas_consumed {
                Some(c.gas_consumed)
            } else {
                None
            },
            return_data: if cfg.return_data {
                Some(c.return_data)
            } else {
                None
            },
            input: if cfg.input { Some(c.input) } else { None },
            executed_code: if cfg.executed_code {
                Some(c.executed_code)
            } else {
                None
            },
            suicide: if cfg.suicide { Some(c.suicide) } else { None },
            keccak_preimages: if cfg.keccak_preimages {
                Some(c.keccak_preimages)
            } else {
                None
            },
            status_failed: if cfg.status_failed {
                Some(c.status_failed)
            } else {
                None
            },
            status_reverted: if cfg.status_reverted {
                Some(c.status_reverted)
            } else {
                None
            },
            failure_reason: if cfg.failure_reason {
                Some(c.failure_reason)
            } else {
                None
            },
            state_reverted: if cfg.state_reverted {
                Some(c.state_reverted)
            } else {
                None
            },
            begin_ordinal: if cfg.begin_ordinal {
                Some(c.begin_ordinal)
            } else {
                None
            },
            end_ordinal: if cfg.end_ordinal {
                Some(c.end_ordinal)
            } else {
                None
            },
            storage_changes: Some(
                c.storage_changes
                    .into_iter()
                    .map(|s| self.extract_storage_change_for_transaction_trace(s))
                    .collect(),
            ),
            balance_changes: Some(
                c.balance_changes
                    .into_iter()
                    .map(|b| self.extract_balance_change_for_transaction_trace(b))
                    .collect(),
            ),
            code_changes: Some(
                c.code_changes
                    .into_iter()
                    .map(|cc| self.extract_code_change_for_transaction_trace(cc))
                    .collect(),
            ),
            nonce_changes: Some(
                c.nonce_changes
                    .into_iter()
                    .map(|n| self.extract_nonce_change_for_transaction_trace(n))
                    .collect(),
            ),
            gas_changes: Some(
                c.gas_changes
                    .into_iter()
                    .map(|g| self.extract_gas_change_for_transaction_trace(g))
                    .collect(),
            ),
            account_creations: Some(
                c.account_creations
                    .into_iter()
                    .map(|a| self.extract_account_creation_for_transaction_trace(a))
                    .collect(),
            ),
        }
    }

    fn extract_storage_change_for_transaction_trace(&self, s: StorageChange) -> ExtractedStorageChange {
        let cfg = &self.block.transaction_traces.calls.storage_changes;
        ExtractedStorageChange {
            address: if cfg.address { Some(s.address) } else { None },
            key: if cfg.key { Some(s.key) } else { None },
            old_value: if cfg.old_value {
                Some(s.old_value)
            } else {
                None
            },
            new_value: if cfg.new_value {
                Some(s.new_value)
            } else {
                None
            },
            ordinal: if cfg.ordinal { Some(s.ordinal) } else { None },
        }
    }

    fn extract_balance_change_for_transaction_trace(&self, b: BalanceChange) -> ExtractedBalanceChange {
        let cfg = &self.block.transaction_traces.calls.balance_changes;
        ExtractedBalanceChange {
            address: if cfg.address { Some(b.address) } else { None },
            old_value: if cfg.old_value { b.old_value } else { None },
            new_value: if cfg.new_value { b.new_value } else { None },
            reason: if cfg.reason { Some(b.reason) } else { None },
            ordinal: if cfg.ordinal { Some(b.ordinal) } else { None },
        }
    }

    fn extract_nonce_change_for_transaction_trace(&self, n: NonceChange) -> ExtractedNonceChange {
        let cfg = &self.block.transaction_traces.calls.nonce_changes;
        ExtractedNonceChange {
            address: if cfg.address { Some(n.address) } else { None },
            old_value: if cfg.old_value {
                Some(BigInt {
                    bytes: n.old_value.to_be_bytes().to_vec(),
                })
            } else {
                None
            },
            new_value: if cfg.new_value {
                Some(BigInt {
                    bytes: n.new_value.to_be_bytes().to_vec(),
                })
            } else {
                None
            },
            ordinal: if cfg.ordinal { Some(n.ordinal) } else { None },
        }
    }

    fn extract_code_change_for_transaction_trace(&self, cc: CodeChange) -> ExtractedCodeChange {
        let cfg = &self.block.transaction_traces.calls.code_changes;
        ExtractedCodeChange {
            address: if cfg.address { Some(cc.address) } else { None },
            old_hash: if cfg.old_hash {
                Some(cc.old_hash)
            } else {
                None
            },
            old_code: if cfg.old_code {
                Some(cc.old_code)
            } else {
                None
            },
            new_hash: if cfg.new_hash {
                Some(cc.new_hash)
            } else {
                None
            },
            new_code: if cfg.new_code {
                Some(cc.new_code)
            } else {
                None
            },
            ordinal: cc.ordinal,
        }
    }

    fn extract_gas_change_for_transaction_trace(&self, g: GasChange) -> ExtractedGasChange {
        let cfg = &self.block.transaction_traces.calls.gas_changes;
        ExtractedGasChange {
            old_value: if cfg.old_value {
                Some(g.old_value)
            } else {
                None
            },
            new_value: if cfg.new_value {
                Some(g.new_value)
            } else {
                None
            },
            reason: if cfg.reason { Some(g.reason) } else { None },
            ordinal: if cfg.ordinal { Some(g.ordinal) } else { None },
        }
    }

    fn extract_account_creation_for_transaction_trace(&self, a: AccountCreation) -> ExtractedAccountCreations {
        let cfg = &self.block.transaction_traces.calls.account_creations;
        ExtractedAccountCreations {
            account: if cfg.account { Some(a.account) } else { None },
            ordinal: if cfg.ordinal { Some(a.ordinal) } else { None },
        }
    }

    fn extract_set_code_authorization(
        &self,
        s: SetCodeAuthorization,
    ) -> ExtractedSetCodeAuthorization {
        let cfg = &self.block.transaction_traces.set_code_authorizations;
        ExtractedSetCodeAuthorization {
            discarded: if cfg.discarded { Some(s.discarded) } else { None },
            chain_id: if cfg.chain_id { Some(s.chain_id) } else { None },
            address: if cfg.address { Some(s.address) } else { None },
            nonce: if cfg.nonce { Some(s.nonce) } else { None },
            v: if cfg.v { Some(s.v) } else { None },
            r: if cfg.r { Some(s.r) } else { None },
            s: if cfg.s { Some(s.s) } else { None },
            authority: if cfg.authority { s.authority } else { None },
        }
    }
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
