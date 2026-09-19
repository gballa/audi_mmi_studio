//! mmi-media: FAT32 media image builder, volume splitter, and pre-flight simulator.

pub mod layout;
pub mod builder;
pub mod simulator;

pub use layout::{Fat32Constraints, MediaSanitizationReport, MediaSanitizer, MediaVolume, VolumeSplitter};
pub use builder::{MediaBuildResult, MediaBuilder};
pub use simulator::{PreFlightSimulator, SimulationReport, SimulationStep, UpdateState};
