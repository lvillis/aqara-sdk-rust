//! Aqara Open API SDK.
//!
//! ## Quick start (async)
//! ```no_run
//! # #[cfg(feature = "async")]
//! # async fn demo() -> Result<(), aqara::Error> {
//! use aqara::{types::Credentials, Client};
//!
//! let client = Client::builder(Credentials::new("APP_ID", "KEY_ID", "APP_KEY"))
//!     .access_token("ACCESS_TOKEN")
//!     .build()?;
//!
//! let resp = client
//!     .positions()
//!     .list(aqara::types::positions::ListPositionsParams::default())
//!     .await?;
//! println!("requestId={} message={}", resp.request_id(), resp.message());
//! println!("result={:?}", resp.result());
//! # Ok(())
//! # }
//! ```
//!
//! ## Quick start (blocking)
//! ```no_run
//! # #[cfg(feature = "blocking")]
//! # fn demo() -> Result<(), aqara::Error> {
//! use aqara::{types::Credentials, BlockingClient};
//!
//! let client = BlockingClient::builder(Credentials::new("APP_ID", "KEY_ID", "APP_KEY"))
//!     .build_blocking()?;
//!
//! let resp = client.positions().list(aqara::types::positions::ListPositionsParams::default())?;
//! println!("requestId={} message={}", resp.request_id(), resp.message());
//! println!("result={:?}", resp.result());
//! # Ok(())
//! # }
//! ```

#![forbid(unsafe_code)]

#[cfg(all(feature = "rustls", feature = "native-tls"))]
compile_error!("Enable only one of: rustls, native-tls");

#[cfg(not(any(feature = "async", feature = "blocking")))]
compile_error!("Enable at least one of: async, blocking");

pub mod api;
mod auth;
mod client;
mod error;
mod transport;
pub mod types;
mod util;

#[cfg(feature = "blocking")]
pub use crate::client::BlockingClient;
#[cfg(feature = "async")]
pub use crate::client::Client;
pub use crate::client::ClientBuilder;
pub use crate::error::{Error, ErrorKind, Result};
