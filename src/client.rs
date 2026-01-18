//! High-level Nessus client implementation.
//!
//! This module provides [`NessusClient`], which handles:
//!
//! - Fetching the X-API token from `nessus6.js`
//! - Logging in to obtain a session token
//! - Launching scans with retry and backoff
//! - Parallel execution of multiple scans
//!
//! ## Example
//!
//! ```no_run
//! use nessus_launcher::{NessusClient, NessusConfig, Result};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let config = NessusConfig::from_env()?;
//!     let client = NessusClient::new(config)?;
//!     client.launch_scans_parallel(vec![5, 8, 11]).await?;
//!     Ok(())
//! }
//! ```

use crate::{NessusConfig, NessusError, Result};
use futures::stream::{FuturesUnordered, StreamExt};
use reqwest::{Client, ClientBuilder};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde_json::Value;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio_retry::strategy::ExponentialBackoff;
use tokio_retry::Retry;
use tracing::{info, error};

/// A high-level asynchronous client for interacting with a Nessus server.
///
/// The client is responsible for:
///
/// - Fetching the X-API token from the Nessus JavaScript file
/// - Logging in with username/password to obtain a session token
/// - Launching scans with retry and backoff
/// - Running multiple scan launches in parallel
pub struct NessusClient {
    client: Client,
    config: NessusConfig,
}

impl NessusClient {
    /// Create a new [`NessusClient`] from the given configuration.
    ///
    /// # Errors
    ///
    /// Returns [`NessusError::Other`] if the underlying HTTP client cannot be built.
    pub fn new(config: NessusConfig) -> Result<Self> {
        let client = ClientBuilder::new()
            .build()
            .map_err(|e| NessusError::Other(format!("Failed to build HTTP client: {e}")))?;

        Ok(Self { client, config })
    }

    /// Fetch the X-API token by requesting `nessus6.js` from the Nessus server.
    ///
    /// This method parses the JavaScript file to extract the `getApiToken` value.
    ///
    /// # Errors
    ///
    /// Returns [`NessusError::Other`] if the token cannot be found or parsed.
    async fn get_x_api_token(&self) -> Result<String> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| NessusError::Other(format!("Time error: {e}")))?
            .as_secs()
            .to_string();

        let url = format!("{}/nessus6.js?v={}", self.config.host, timestamp);

        let body = self.client.get(&url).send().await?.text().await?;

        let parts = body.split(":\"").collect::<Vec<&str>>();
        let token_part = parts
            .iter()
            .find(|s| s.contains("getApiToken"))
            .ok_or_else(|| NessusError::Other("getApiToken not found in nessus6.js".into()))?;

        let vec3 = token_part.split('"').collect::<Vec<&str>>();
        if vec3.len() < 3 {
            return Err(NessusError::Other(
                "Unexpected format while parsing X-API token".into(),
            ));
        }

        Ok(vec3[2].to_string())
    }

    /// Log in to Nessus using the configured username and password.
    ///
    /// This returns a session token that is used in the `X-Cookie` header.
    ///
    /// # Errors
    ///
    /// Returns [`NessusError::Json`] if the response cannot be parsed,
    /// or [`NessusError::Other`] if the token field is missing.
    async fn login(&self, x_api_token: &str) -> Result<String> {
        let url = format!("{}/session", self.config.host);

        let body = serde_json::json!({
            "username": self.config.username,
            "password": self.config.password,
        });

        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0"));
        headers.insert("X-Api-Token", HeaderValue::from_str(x_api_token).map_err(|e| {
            NessusError::Other(format!("Invalid X-Api-Token header value: {e}"))
        })?);
        headers.insert("content-type", HeaderValue::from_static("application/json"));

        let resp_text = self
            .client
            .post(url)
            .headers(headers)
            .json(&body)
            .send()
            .await?
            .text()
            .await?;

        let v: Value = serde_json::from_str(&resp_text)?;
        let token = v
            .get("token")
            .and_then(|t| t.as_str())
            .ok_or_else(|| NessusError::Other("Missing 'token' field in session response".into()))?;

        Ok(token.to_string())
    }

    /// Launch a single Nessus scan once, without retry.
    ///
    /// # Errors
    ///
    /// Returns [`NessusError::Http`] if the HTTP request fails,
    /// or [`NessusError::Other`] if the response status is not successful.
    async fn launch_scan_once(
        &self,
        scan_id: u32,
        x_api_token: &str,
        x_cookie: &str,
    ) -> Result<()> {
        let url = format!("{}/scans/{}/launch", self.config.host, scan_id);

        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0"));
        headers.insert(
            "X-Api-Token",
            HeaderValue::from_str(x_api_token)
                .map_err(|e| NessusError::Other(format!("Invalid X-Api-Token header: {e}")))?,
        );
        headers.insert(
            "X-Cookie",
            HeaderValue::from_str(x_cookie)
                .map_err(|e| NessusError::Other(format!("Invalid X-Cookie header: {e}")))?,
        );
        headers.insert("content-type", HeaderValue::from_static("application/json"));

        let resp = self.client.post(url).headers(headers).send().await?;

        if !resp.status().is_success() {
            return Err(NessusError::Other(format!(
                "Scan {} launch failed with status {}",
                scan_id,
                resp.status()
            )));
        }

        Ok(())
    }

    /// Launch multiple scans in parallel with retry and exponential backoff.
    ///
    /// Each scan is launched in its own task, and each task uses a retry
    /// strategy with exponential backoff.
    ///
    /// # Errors
    ///
    /// Returns an error if obtaining the X-API token or session token fails.
    /// Individual scan failures are logged but do not abort the entire operation.
    pub async fn launch_scans_parallel(&self, scan_ids: Vec<u32>) -> Result<()> {
        if scan_ids.is_empty() {
            info!("No scan IDs provided; nothing to launch.");
            return Ok(());
        }

        let x_api_token = self.get_x_api_token().await?;
        let session_token = self.login(&x_api_token).await?;
        let x_cookie = format!("token={}", session_token);

        let mut tasks = FuturesUnordered::new();

        for scan_id in scan_ids {
            let client = self.clone();
            let x_api_token = x_api_token.clone();
            let x_cookie = x_cookie.clone();

            tasks.push(tokio::spawn(async move {
                let strategy = ExponentialBackoff::from_millis(500)
                    .max_delay(Duration::from_secs(10))
                    .take(5);

                let result = Retry::spawn(strategy, || async {
                    client.launch_scan_once(scan_id, &x_api_token, &x_cookie).await
                })
                .await;

                match result {
                    Ok(_) => info!("Scan {} launched successfully", scan_id),
                    Err(e) => error!("Scan {} failed after retries: {}", scan_id, e),
                }
            }));
        }

        while let Some(join_result) = tasks.next().await {
            if let Err(e) = join_result {
                error!("Task join error: {}", e);
            }
        }

        Ok(())
    }
}

impl Clone for NessusClient {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            config: self.config.clone(),
        }
    }
}

