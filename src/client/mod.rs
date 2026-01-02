mod builder;

#[cfg(feature = "async")]
mod async_client;

#[cfg(feature = "blocking")]
mod blocking_client;

pub use builder::ClientBuilder;

#[cfg(feature = "async")]
pub use async_client::Client;

#[cfg(feature = "blocking")]
pub use blocking_client::BlockingClient;
