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
