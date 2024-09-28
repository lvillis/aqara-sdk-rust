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

- [ ] Location management interface
- [ ] Equipment distribution network interface
- [ ] Device management interface
- [ ] Device resource interface
- [ ] Infrared device management interface
- [ ] Device firmware management interface
- [ ] Linkage configuration query interface
- [ ] Automation management interface
- [ ] Scene management interface
- [ ] Condition set management interface
- [x] Voice control interface


## Usage

```toml
[dependencies]
aqara = { version="0.1.0", default-features = false, features = ["singapore"] }
```

```rust
#[tokio::main]
async fn main() {
    let config = AqaraConfig {
        access_token: "your_access_token".to_string(),
        app_id: "your_app_id".to_string(),
        key_id: "your_key_id".to_string(),
        app_key: "your_app_key".to_string(),
    };

    let client = AqaraClient::new(config);
    let response = client.query_position_info(Some("parent_position_id"), Some(1), Some(30)).await;
    
    match response {
        Ok(data) => println!("Response: {}", data),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```