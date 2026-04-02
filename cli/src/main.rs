use anyhow::Result;
use clickhouse::Client;
use config::{Case, Config, Environment, File};
use engine::mapping::{load_mappings, resolve_and_insert};
use futures_util::StreamExt;
use ingestor::stream_blocks_mock;
use settings::AppConfig;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let config: AppConfig = Config::builder()
        .add_source(File::with_name("crates/settings/default.toml"))
        .add_source(File::with_name("config.toml"))
        .add_source(Environment::default().convert_case(Case::Snake))
        .build()
        .unwrap()
        .try_deserialize()
        .expect("Failed to deserialize config");
    println!(
        "Config loaded: streaming blocks {} → {}",
        config.start_block, config.end_block
    );

    let mappings = load_mappings("mapping.toml")?;

    let client = Client::default()
        .with_url("http://localhost:8123")
        .with_database("users_db")
        .with_user("user")
        .with_password("admin");


    let columns_ddl = mappings
        .values()
        .map(|col| format!("    `{}` Nullable(String)", col))
        .collect::<Vec<_>>()
        .join(",\n");

    let create_sql = format!(
        "CREATE TABLE IF NOT EXISTS blocks (\n{}\n) ENGINE = MergeTree() ORDER BY tuple()",
        columns_ddl
    );
    client.query(&create_sql).execute().await?;
    println!("Table 'blocks' is ready in ClickHouse");

    let stream = stream_blocks_mock(&config);
    tokio::pin!(stream);

    let mut block_count = 0u64;

    while let Some(result) = stream.next().await {
        let raw_block = result.map_err(|e| anyhow::anyhow!("{}", e))?;
        let block_num = raw_block.number;

        let extracted = config
            .extract(raw_block)
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?;

        resolve_and_insert(&extracted, &mappings, &client, "blocks").await?;

        block_count += 1;
        println!("Inserted block #{} ({} total)", block_num, block_count);
    }

    println!("Done. Inserted {} block(s) into ClickHouse.", block_count);
    Ok(())
}
