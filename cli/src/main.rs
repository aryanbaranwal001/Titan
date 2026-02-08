use dotenv::dotenv;
use ingestion::{connect_to_firehose, get_firehose_token, stream_blocks};
use settings::AppConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv()?;
    let conf = AppConfig::new()?;
    let token = get_firehose_token(conf.firehose_api_key.clone(), &conf.auth_endpoint).await?;
    let s = conf.endpoint_url.clone();
    let str: &'static str = Box::leak(s.into_boxed_str());
    let ingestor_client = connect_to_firehose(token, str).await?;
    match stream_blocks(ingestor_client).await {
        Ok(_) => {}
        Err(e) => {
            println!("streaming error: {:#?}", e);
        }
    }
    Ok(())
}
