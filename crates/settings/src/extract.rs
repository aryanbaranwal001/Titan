use crate::AppConfig;
use prost_types::Timestamp;
use proto_build::sf::ethereum::r#type::v2::{BigInt, Block, BlockHeader, Uint64NestedArray};
use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Debug)]
pub struct ExtractedBlock {
    pub hash: Option<Vec<u8>>,
    pub number: Option<u64>,
    pub size: Option<u64>,
    pub detail_level: Option<DetailLevel>,
    pub ver: Option<i32>,
    pub blockheader: Option<ExtractedBlockHeader>,
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
