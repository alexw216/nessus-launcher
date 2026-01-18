# nessus-cli

[![Crates.io](https://img.shields.io/crates/v/nessus-cli.svg)](https://crates.io/crates/nessus-cli)
[![Docs.rs](https://docs.rs/nessus-cli/badge.svg)](https://docs.rs/nessus-cli)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

A fast, reliable command‑line interface for launching Nessus scans using the
[`nessus-launcher`](https://crates.io/crates/nessus-launcher) Rust library.

This CLI is designed for automation, CI pipelines, and operational workflows where
you need a clean, scriptable interface to Nessus.

---

## ✨ Features

- 🚀 Launch one or many Nessus scans
- 🔄 Automatic retry with exponential backoff
- ⚡ Parallel execution
- 🧩 `.env` configuration support
- 📊 Structured logging (`tracing`)
- 🛠 Built on the `nessus-launcher` async Rust library
- 🧪 Fully scriptable for CI/CD

---

## 📦 Installation

Install from crates.io:

```bash
cargo install nessus-cli


Verify installation:

nessus-cli --help


🚀 Quick Start
Launch scans directly

nessus-cli --scan 5 --scan 8



Or configure via .env
Create a .env file:
NESSUS_HOST=https://nessus.example.com
NESSUS_USERNAME=admin
NESSUS_PASSWORD=secret
DEFAULT_SCAN_IDS=5,8,11


Then simply run:

nessus-cli


🔧 Command-Line Options

nessus-cli [OPTIONS]



Common flags
Flag	Description
--scan	Launch a specific scan (repeatable)
--parallel	Override parallelism level
--retries	Override retry count
--delay	Override retry backoff delay
-v, --verbose	Increase log verbosity
-h, --help	Show help message

🧩 Example: Launch Multiple Scans

nessus-cli \
  --scan 12 \
  --scan 15 \
  --scan 22 \
  --parallel 3 \
  --retries 5


🛠 Configuration

The CLI automatically loads environment variables from:

.env file (if present)

system environment variables

Supported variables:

NESSUS_HOST=
NESSUS_USERNAME=
NESSUS_PASSWORD=
DEFAULT_SCAN_IDS=
PARALLELISM=
RETRY_COUNT=
RETRY_DELAY_MS=


🏗 Project Structure

nessus-cli/
├── Cargo.toml
└── src/
    └── main.rs


The CLI is a separate crate built on top of the nessus-launcher library.

📚 Documentation

Full CLI documentation:

https://docs.rs/nessus-cli

Library documentation:

https://docs.rs/nessus-launcher (docs.rs in Bing)


📄 License
This project is licensed under the MIT License.
See LICENSE for details.


