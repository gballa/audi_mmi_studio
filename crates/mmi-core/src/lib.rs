//! mmi-core: Core storage, SourceStore immutability, Content-Addressed Storage, and MMIProject.

pub mod error;
pub mod immutable_path;
pub mod source_store;
pub mod cas;
pub mod project;
pub mod stage;
pub mod extractor;

pub use error::CoreError;
pub use immutable_path::ImmutablePath;
pub use source_store::SourceStore;
pub use cas::ContentAddressedStore;
pub use project::{
    ConfidenceLevel, MMIProject, ModuleRecord, PlatformVariant, ProjectMetadata,
    SignedArtefactRecord, SourceRef, TaggedValue, TargetProfile,
};
pub use stage::{StageEntry, StageStore};
pub use extractor::PackageExtractor;
