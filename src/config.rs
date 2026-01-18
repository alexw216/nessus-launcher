//! Configuration handling for `nessus-launcher`.
//!
//! Configuration is typically loaded from environment variables,
//! optionally via a `.env` file using `dotenvy`.
//!
//! ## Environment variables
//!
//! - `NESSUS_HOST` — Base URL of the Nessus server (e.g. `https://nessus.example.com`)
//! - `NESSUS_USERNAME` — Nessus username
//! - `NESSUS_PASSWORD` — Nessus password
//! - `DEFAULT_SCAN_IDS` — Comma-separated list of scan IDs (e.g. `5,8,11`)

use crate::{NessusError, Result};
use dotenvy::dotenv;
use std::env;

/// Configuration for connecting to a Nessus server.
#[derive(Debug, Clone)]
pub struct NessusConfig {
    /// Base URL of the Nessus server, e.g. `https://nessus.example.com`.
    pub host: String,
    /// Nessus username.
    pub username: String,
    /// Nessus password.
    pub password: String,
}

impl NessusConfig {
    /// Load configuration from environment variables.
    ///
    /// This will call `dotenv().ok()` to load variables from a `.env` file if present.
    ///
    /// # Errors
    ///
    /// Returns [`NessusError::Config`] if any required variable is missing.
    pub fn from_env() -> Result<Self> {
        dotenv().ok();

        let host = env::var("NESSUS_HOST")
            .map_err(|_| NessusError::Config("Missing NESSUS_HOST".into()))?;
        let username = env::var("NESSUS_USERNAME")
            .map_err(|_| NessusError::Config("Missing NESSUS_USERNAME".into()))?;
        let password = env::var("NESSUS_PASSWORD")
            .map_err(|_| NessusError::Config("Missing NESSUS_PASSWORD".into()))?;

        Ok(Self {
            host,
            username,
            password,
        })
    }

    /// Load default scan IDs from the `DEFAULT_SCAN_IDS` environment variable.
    ///
    /// Example:
    ///
    /// ```env
    /// DEFAULT_SCAN_IDS=5,8,11
    /// ```
    ///
    /// # Returns
    ///
    /// A vector of scan IDs. Invalid entries are ignored.
    pub fn default_scan_ids_from_env() -> Vec<u32> {
        dotenv().ok();

        let ids = env::var("DEFAULT_SCAN_IDS").unwrap_or_default();
        ids.split(',')
            .filter_map(|s| s.trim().parse::<u32>().ok())
            .collect()
    }
}

