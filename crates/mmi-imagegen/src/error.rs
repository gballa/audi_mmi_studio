//! Error definitions for AI asset authoring and Egress Airlock.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImageGenError {
    #[error("Egress denied (ERR_EGRESS_DENIED): {0}")]
    EgressDenied(String),

    #[error("Network is disabled for this project (ERR_NETWORK_DISABLED): {0}")]
    NetworkDisabled(String),

    #[error("Provider error: {0}")]
    ProviderError(String),

    #[error("I/O or CAS error: {0}")]
    Core(#[from] mmi_core::CoreError),
}
