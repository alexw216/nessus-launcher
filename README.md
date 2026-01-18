# nessus-launcher

[![Crates.io](https://img.shields.io/crates/v/nessus-launcher.svg)](https://crates.io/crates/nessus-launcher)
[![Docs.rs](https://docs.rs/nessus-launcher/badge.svg)](https://docs.rs/nessus-launcher)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
[![CI](https://github.com/alexw216/nessus-launcher/actions/workflows/ci.yml/badge.svg)](https://github.com/alexw216/nessus-launcher/actions)

A production‑grade Rust library and CLI for launching Nessus scans with:

- 🔄 Automatic retry with exponential backoff  
- ⚡ Parallel scan launching  
- 🧩 `.env` configuration  
- 📊 Structured logging (`tracing`)  
- 🧰 Reusable library + CLI binary  
- ❄️ Reproducible builds via Nix flake  

Designed for automation, CI pipelines, and secure operational workflows.

---

## 📦 Installation

### Library (crates.io)

```toml
[dependencies]
nessus-launcher = "0.1"

CLI (GitHub Releases)
cargo install nessus-cli


Quick Start (CLI)
Launch scans explicitly:
nessus-cli --scan 5 --scan 8


Or configure via .env:
NESSUS_HOST=https://nessus.example.com
NESSUS_USERNAME=admin
NESSUS_PASSWORD=secret
DEFAULT_SCAN_IDS=5,8,11


Then simply run:

nessus-cli


Library Example

use nessus_launcher::{NessusClient, NessusConfig, Result};

#[toktokio::main]
async fn main() -> Result<()> {
    let config = NessusConfig::from_env()?;
    let client = NessusClient::new(config)?;
    client.launch_scans_parallel(vec![5, 8]).await?;
    Ok(())
}


Documentation

Full API docs:
https://docs.rs/nessus-launcher (docs.rs in Bing)

Examples:

cargo run --example basic
cargo run --example parallel


Project Structure

nessus-launcher/
├── src/                # Library source code
├── nessus-cli/         # CLI binary crate
├── examples/           # docs.rs examples
├── tests/              # minimal tests
├── scripts/            # build scripts
├── .github/workflows/  # CI pipeline
├── Makefile            # build/run/release automation
└── flake.nix           # reproducible Nix environment


Development

Build
gmake build


Run CLI

gmake run ARGS="--scan 5"


Test

cargo test


Format and lint

cargo fmt
cargo clippy --all-targets -- -D warnings


Nix Development Shell

nix develop
cargo build



License

This project is licensed under the MIT License.
See LICENSE for details.

Acknowledgments

This project provides a clean, modern, async Rust interface for Nessus automation workflows, with a focus on reliability, reproducibility, and operational clarity.



