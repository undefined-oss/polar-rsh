//! Rust SDK for Polar.
//!
//! Enable an API version feature to use its namespace, such as
//! `polar_rsh::v2026_04` or `polar_rsh::v2026_10`. Multiple versions can
//! coexist. No API version is enabled by default.
//!
//! The `presence` feature exposes `polar_rsh::presence_rs` and
//! `polar_rsh::Presence` independently of an API version. It is also enabled
//! automatically by every API version feature.

#[cfg(feature = "presence")]
pub use presence_rs::{self, Presence};

mod versions;
// The module deliberately exports nothing when no API features are enabled.
#[allow(unused_imports)]
pub use versions::*;
