//! mmi-core: Core storage, SourceStore immutability, Content-Addressed Storage, and MMIProject.

pub mod error;
pub mod immutable_path;
pub mod source_store;
pub mod cas;
pub mod project;
pub mod stage;
pub mod extractor;
pub mod obd;

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
pub use obd::{
    decode_obd_coolant_temp, decode_obd_rpm, decode_obd_speed, decode_obd_voltage,
    solve_svm_03276, DiagnosticSessionReport, GemActivationReport, SvmResolutionReport,
    VehicleTelemetry, VirtualObdBridge, MODULE_5F_CAN_RX_ID, MODULE_5F_CAN_TX_ID,
    SVM_CHANNEL_15_XOR_CIPHER,
};
