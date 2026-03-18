use crate::mapping::{load_mappings, resolve_and_insert};
use anyhow::Result;
use clickhouse::Client;
use futures_util::stream::{self, Stream, StreamExt};
use settings::structure::ExtractedBlock;
use std::collections::HashMap;

pub mod mapping;

pub struct Engine {
    client: Client,
    mappings: HashMap<String, String>,
    table_name: String,
}

impl Engine {
    pub fn new(client: Client, mappings: HashMap<String, String>, table_name: &str) -> Self {
        Self {
            client,
            mappings,
            table_name: table_name.to_string(),
        }
    }

    pub async fn run(&self) -> Result<()> {
        let mut stream = Self::get_block_stream().await;

        while let Some(block) = stream.next().await {
            match resolve_and_insert(&block, &self.mappings, &self.client, &self.table_name).await {
                Ok(_) => println!(
                    "Inserted block {} successfully",
                    block.number.unwrap_or_default()
                ),
                Err(e) => eprintln!("Failed to insert block: {:?}", e),
            }
        }

        Ok(())
    }

    async fn get_block_stream() -> impl Stream<Item = ExtractedBlock> {
        // This is a placeholder for the actual block stream.
        // In a real application, this would come from a Firehose connection or other source.
        let blocks = vec![ExtractedBlock {
            hash: Some(b"hash1".to_vec()),
            number: Some(1),
            size: Some(1024),
            detail_level: None,
            ver: Some(1),
            blockheader: Some(settings::structure::ExtractedBlockHeader {
                parent_hash: Some(b"parenthash1".to_vec()),
                uncle_hash: None,
                coinbase: None,
                state_root: None,
                transactions_root: None,
                receipt_root: None,
                logs_bloom: None,
                difficulty: None,
                total_difficulty: None,
                number: Some(1),
                gas_limit: None,
                gas_used: None,
                timestamp: None,
                extra_data: None,
                mix_hash: None,
                nonce: None,
                hash: Some(b"hash1".to_vec()),
                base_fee_per_gas: None,
                withdrawals_root: None,
                tx_dependency: None,
                blob_gas_used: None,
                excess_blob_gas: None,
                parent_beacon_root: None,
                requests_hash: None,
            }),
            system_calls: None,
            uncles: None,
            transaction_traces: Some(vec![settings::structure::ExtractedTransactionTraces {
                to: None,
                nonce: None,
                gas_price: None,
                gas_limit: None,
                value: None,
                input: None,
                v: None,
                r: None,
                s: None,
                gas_used: None,
                r#type: None,
                max_fee_per_gas: None,
                max_priority_fee_per_gas: None,
                index: Some(0),
                hash: Some(b"txhash1".to_vec()),
                from: None,
                return_data: None,
                public_key: None,
                begin_ordinal: None,
                end_ordinal: None,
                status: None,
                blob_gas: None,
                blob_gas_fee_cap: None,
                blob_hashes: None,
                access_list: None,
                receipt: None,
                calls: None,
                set_code_authorizations: None,
            }]),
            balance_changes: None,
            code_changes: None,
            withdrawals: None,
        }];
        stream::iter(blocks)
    }
}

// This is an example of how you might run the engine.
// You would typically call this from your application's main function.
pub async fn start_engine() -> Result<()> {
    let mappings = load_mappings("mapping.toml")?;

    let client = Client::default()
        .with_url("http://localhost:8123")
        .with_database("default");

    let engine = Engine::new(client, mappings, "blocks");
    engine.run().await?;

    Ok(())
}
