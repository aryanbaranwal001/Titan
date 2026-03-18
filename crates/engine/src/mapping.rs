use anyhow::Result;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use settings::structure::ExtractedBlock;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub mappings: HashMap<String, String>,
}

pub fn load_mappings(path: &str) -> Result<HashMap<String, String>> {
    let content = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
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
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => format!("'{}'", s.replace('\'', "\\'")),
        Value::Array(arr) => format!("'{}'", serde_json::to_string(arr).unwrap_or_default().replace('\'', "\\'")),
        Value::Object(obj) => format!("'{}'", serde_json::to_string(obj).unwrap_or_default().replace('\'', "\\'")),
    }
}
