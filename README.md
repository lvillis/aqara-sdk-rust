<div align=right>Table of Contents↗️</div>

<h1 align=center><code>aqara-sdk-rust</code></h1>

<p align=center>Aqara SDK for Rust.</p>

<div align=center>
  <a href="https://crates.io/crates/aqara">
    <img src="https://img.shields.io/crates/v/aqara.svg" alt="crates.io version">
  </a>
  <a href="https://crates.io/crates/aqara-sdk-rust">
    <img src="https://img.shields.io/github/repo-size/lvillis/aqara-sdk-rust?style=flat-square&color=328657" alt="crates.io version">
  </a>
  <a href="https://github.com/lvillis/aqara-sdk-rust/actions">
    <img src="https://github.com/lvillis/aqara-sdk-rust/actions/workflows/ci.yaml/badge.svg" alt="build status">
  </a>
  <a href="mailto:lvillis@outlook.com?subject=Thanks%20for%20aqara-sdk-rust!">
    <img src="https://img.shields.io/badge/Say%20Thanks-!-1EAEDB.svg" alt="say thanks">
  </a>
</div>

---

## Features

- [x] Auth interface
- [x] Location management interface
- [x] Equipment distribution network interface
- [x] Device management interface
- [x] Device resource interface
- [x] Infrared device management interface
- [x] Device firmware management interface
- [x] Linkage configuration query interface
- [x] Automation management interface
- [x] Scene management interface
- [x] Condition set management interface
- [x] Voice control interface
- [x] Push subscription interface


## Usage

```toml
[dependencies]
aqara = "0.1.0"
```

```rust
use aqara::types::{Credentials, Endpoint};
use aqara::Client;

#[tokio::main]
async fn main() -> Result<(), aqara::Error> {
    let client = Client::builder(Credentials::new("APP_ID", "KEY_ID", "APP_KEY"))
        .endpoint(Endpoint::Singapore)
        .access_token("ACCESS_TOKEN")
        .build()?;

    let resp = client.positions().list(Default::default()).await?;
    println!("requestId={} message={}", resp.request_id(), resp.message());
    println!("result={:?}", resp.result());

    Ok(())
}
```
