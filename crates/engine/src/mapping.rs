use anyhow::Result;
use serde::Deserialize;
use serde_json::Value;
use settings::structure::ExtractedBlock;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Mappings {
    pub mappings: HashMap<String, String>,
}

pub fn load_mappings(path: &str) -> Result<HashMap<String, String>> {
    let content = std::fs::read_to_string(path)?;
    let config: Mappings = toml::from_str(&content)?;
    Ok(config.mappings)
}

pub async fn resolve_and_insert(
    block: &ExtractedBlock,
    mappings: &HashMap<String, String>,
    client: &clickhouse::Client,
    table_name: &str,
) -> Result<()> {
    let block_value = serde_json::to_value(block)?;

    let mut columns = Vec::new();
    let mut values = Vec::new();

    for (source, target) in mappings {
        if let Some(value) = block_value.pointer(source) {
            columns.push(target.clone());
            values.push(format_sql_value(value));
        }
    }

    if columns.is_empty() {
        return Ok(());
    }

    let columns_sql = columns.join(", ");
    let values_sql = values.join(", ");
    let sql = format!(
        "INSERT INTO {} ({}) VALUES ({})",
        table_name, columns_sql, values_sql
    );

    client.query(&sql).execute().await?;

    Ok(())
}

fn format_sql_value(value: &Value) -> String {
    match value {
        Value::Null => "NULL".to_string(),

        Value::Bool(b) => format!("'{}'", b),

        Value::Number(n) => format!("'{}'", n),

        Value::String(s) => format!("'{}'", s.replace('\'', "\\'")),

        Value::Array(arr) => {
            let is_byte_array =
                !arr.is_empty() && arr.iter().all(|v| v.as_u64().map_or(false, |n| n <= 255));

            if is_byte_array {
                let hex_str: String = arr
                    .iter()
                    .filter_map(|v| v.as_u64())
                    .map(|b| format!("{:02x}", b as u8))
                    .collect();
                format!("'0x{}'", hex_str)
            } else {
                format!(
                    "'{}'",
                    serde_json::to_string(arr)
                        .unwrap_or_default()
                        .replace('\'', "\\'")
                )
            }
        }

        Value::Object(obj) => format!(
            "'{}'",
            serde_json::to_string(obj)
                .unwrap_or_default()
                .replace('\'', "\\'")
        ),
    }
}

// Todo
// 1. include the types in mappings.toml so we can store data (like bool instead of string) in
//    database
