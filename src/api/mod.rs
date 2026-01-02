//! Service modules.

pub mod auth;
pub mod devices;
pub mod events;
pub mod ifttt;
pub mod ir;
pub mod linkages;
pub mod networking;
pub mod ota;
pub mod positions;
pub mod push;
pub mod resources;
pub mod scenes;
pub mod voice;

#[cfg(feature = "unstable-raw")]
pub mod raw;
