//! mmi-rebuild: Deterministic rebuilding engine, container repackager, and parity verification.

pub mod normalizer;
pub mod packager;
pub mod verifier;
pub mod geo;
pub mod osm_ingest;
pub mod gmp_enrich;
pub mod fldb_compiler;
pub mod firmware_bundle;

pub use normalizer::{NormalizedFileEntry, StageNormalizer};
pub use packager::{BundlePackager, PrecompPackager, RepackageResult, StringCatalogPackager};
pub use verifier::{DeterminismParity, FileParityReport, RebuildVerificationSummary, RebuildVerifier};
pub use geo::{
    access_flags, junction_flags, restriction_types, BoundingBox, FixedPoint32, GeoError,
    IrDataset, IrEdge, IrNode, IrPoi, IrTurnRestriction, RegionalProfile, WebMercatorPoint,
    Wgs84Point, FIXED_POINT_SCALE_I32, WGS84_A,
};
pub use osm_ingest::{
    classify_frc, is_navigable_way, lane_guidance_mask, parse_access_flags, parse_lane_count,
    parse_speed_limits, parse_turn_lanes, statutory_fallback_speed, CountryCode, OsmIngestConfig,
    OsmIngestError, OsmIngestPipeline,
};
pub use gmp_enrich::{
    CachedPoiRecord, EvConnectorType, GeocodingLocationType, GmpCacheManager, GmpClient, GmpEnrichmentPipeline,
    GmpError, GmpGeocodingResult, GmpPlace, GmpPlacesSearchRequest, LiveGmpClient,
    OfflineFixtureGmpClient, PLACES_API_FIELD_MASK, TOS_CACHE_MAX_AGE_SECONDS,
};
pub use fldb_compiler::{
    compile_fldb_database, create_fldb_page, resolve_svm_03276, simulate_svm_03175_rehash,
    split_into_volumes, FldbCompilerPipeline, MapCompileResult, CHECKSUM_CHUNK_SIZE,
    FLDB_HEADER_SIZE, FLDB_MAGIC, FLDB_PAGE_SIZE, FLDB_PAYLOAD_SIZE, FLDB_TRAILER_SYNC,
    GDB_MAGIC, GDB_VERSION, MAX_VOLUME_BYTES, SVM_CHANNEL_15_XOR_CIPHER,
};
pub use firmware_bundle::{
    build_gem_screen_esd, BundleFileRecord, FirmwareBundleConfig, FirmwareBundlePipeline,
    FirmwareBundleReport, GemWidget, PartitionUsage, DEFAULT_RELEASE, DEFAULT_TRAIN,
    DEFAULT_VARIANT, SAFETY_POLICY_BANNER,
};


