//! Launch multiple scans in parallel.
//!
//! Run with:
//!     cargo run --example parallel

use nessus_launcher::{NessusClient, NessusConfig, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let config = NessusConfig::from_env()?;
    let client = NessusClient::new(config)?;

    client.launch_scans_parallel(vec![5, 8, 11]).await?;
    Ok(())
}

