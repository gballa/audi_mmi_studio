//! mmi-imagegen: Single-network-crate boundary for AI asset authoring and Egress Airlock.

pub mod error;
pub mod airlock;
pub mod provider;

pub use error::ImageGenError;
pub use airlock::{EgressAirlock, EgressAuditEntry};
pub use provider::{
    GeminiImageProvider, GeneratedAsset, ImageEditProvider, OfflineMockProvider,
    ProviderCapabilities,
};
