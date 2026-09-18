//! mmi-validation: 6-tier validation engine, hardware target profiles, and build readiness verification.

pub mod profile;
pub mod levels;
pub mod engine;

pub use profile::{MmiGeneration, TargetProfile};
pub use levels::{FindingSeverity, ValidationFinding, ValidationLevel};
pub use engine::{BuildReadinessStatus, ValidationEngine, ValidationReport};
