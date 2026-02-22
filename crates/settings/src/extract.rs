use crate::AppConfig;
use crate::cfg::SystemCallCfg;
use prost_types::Timestamp;
use proto_build::sf::ethereum::r#type::v2::{
    BigInt, Block, BlockHeader, Call, StorageChange, Uint64NestedArray,
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
    pub reason: Option<i32>,
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
            storage_changes: None,
            balance_changes: None,
            code_changes: None,
            nonce_changes: None,
            gas_changes: None,
            account_creations: None,
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

        write!(f, "  }}")
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
