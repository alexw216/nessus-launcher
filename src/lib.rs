//! # nessus-launcher
//!
//! A high-level Rust library for launching Nessus scans with:
//!
//! - Automatic retry with exponential backoff
//! - Parallel scan launching
//! - Structured logging via `tracing`
//! - Configuration via environment variables / `.env`
//!
//! ## Quick example
//!
//! ```no_run
//! use nessus_launcher::{NessusClient, NessusConfig, Result};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let config = NessusConfig::from_env()?;
//!     let client = NessusClient::new(config)?;
//!
//!     // Launch scans 5 and 8 in parallel
//!     client.launch_scans_parallel(vec![5, 8]).await?;
//!     Ok(())
//! }
//! ```

mod client;
mod config;
mod error;

pub use client::NessusClient;
pub use config::NessusConfig;
pub use error::{NessusError, Result};

