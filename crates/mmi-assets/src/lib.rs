//! mmi-assets: Asset decoding, precomp conversion, font analysis, thumbnailing, and conformance pipeline.

pub mod decoder;
pub mod font;
pub mod thumbnail;
pub mod catalog;
pub mod conformance;
pub mod replacement;
pub mod strings;
pub mod overflow;

pub use decoder::{AssetDecoder, DecodedBitmap};
pub use font::{FontInfo, FontInspector};
pub use thumbnail::ThumbnailGenerator;
pub use catalog::AssetCataloger;
pub use conformance::{AssetConstraints, ConformancePipeline, ConformanceVerdict};
pub use replacement::{AssetReplacer, ReplacementRecord};
pub use strings::{EncodingType, StringCatalog, StringEntry};
pub use overflow::{FontMetricsEngine, OverflowPredictor, OverflowReport, TextMetrics, UiBoundingBox};

