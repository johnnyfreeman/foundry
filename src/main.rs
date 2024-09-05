mod config;
mod ea;
mod os;
mod ui;
mod user_management;

use crate::config::Config;
use crate::ea::EaEngine;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "examples/example_config.toml".to_string());
    let config = Config::load(&config_path)?;

    EaEngine::new(config).run().await?;

    Ok(())
}
