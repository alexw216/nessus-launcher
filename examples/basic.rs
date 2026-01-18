//! Basic example: launch a single Nessus scan.
//!
//! Run with:
//!     cargo run --example basic

use nessus_launcher::{NessusClient, NessusConfig, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let config = NessusConfig::from_env()?;
    let client = NessusClient::new(config)?;

    client.launch_scans_parallel(vec![5]).await?;
    Ok(())
}

