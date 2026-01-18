//! Command-line interface for the `nessus-launcher` library.
//!
//! This binary provides a convenient way to launch Nessus scans from the shell.
//!
//! ## Examples
//!
//! Launch scans 5 and 8 explicitly:
//!
//! ```bash
//! nessus-cli --scan 5 --scan 8
//! ```
//!
//! Use default scan IDs from `DEFAULT_SCAN_IDS` in `.env`:
//!
//! ```bash
//! nessus-cli
//! ```

use clap::Parser;
use dotenvy::dotenv;
use nessus_launcher::{NessusClient, NessusConfig, Result};
use tracing::info;
use tracing_subscriber::FmtSubscriber;

/// Command-line arguments for the Nessus CLI.
#[derive(Parser, Debug)]
#[command(name = "nessus-cli")]
#[command(about = "Launch Nessus scans via CLI")]
struct Cli {
    /// One or more scan IDs to launch.
    ///
    /// If omitted, the CLI will use `DEFAULT_SCAN_IDS` from the environment.
    #[arg(long, num_args = 1..)]
    scan: Option<Vec<u32>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .map_err(|e| nessus_launcher::NessusError::Other(format!("Failed to set logger: {e}")))?;

    let args = Cli::parse();

    let config = NessusConfig::from_env()?;
    let client = NessusClient::new(config)?;

    let scan_ids = match args.scan {
        Some(ids) => ids,
        None => NessusConfig::default_scan_ids_from_env(),
    };

    info!("Launching scans: {:?}", scan_ids);

    client.launch_scans_parallel(scan_ids).await
}

