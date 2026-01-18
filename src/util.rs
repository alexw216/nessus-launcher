use dotenvy::dotenv;
use std::env;

use crate::models::NessusConfig;

pub fn load_config() -> NessusConfig {
    dotenv().ok();

    NessusConfig {
        host: env::var("NESSUS_HOST").expect("Missing NESSUS_HOST"),
        username: env::var("NESSUS_USERNAME").expect("Missing NESSUS_USERNAME"),
        password: env::var("NESSUS_PASSWORD").expect("Missing NESSUS_PASSWORD"),
    }
}

pub fn load_default_scan_ids() -> Vec<u32> {
    dotenv().ok();

    let ids = env::var("DEFAULT_SCAN_IDS").unwrap_or_default();
    ids.split(',')
        .filter_map(|s| s.trim().parse::<u32>().ok())
        .collect()
}

