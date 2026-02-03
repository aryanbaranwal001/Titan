use dotenv::dotenv;
use settings::AppConfig;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv()?;
    let configs = AppConfig::new()?;
    println!("configs {:#?}", configs);
    Ok(())
}
