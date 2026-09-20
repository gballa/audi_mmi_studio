//! geo: Geographic coordinate systems, sub-centimeter fixed-point transforms,
//! Morton Z-order curve bit interleaving, and Intermediate Representation (IR) data models.
//!
//! Conforms to Audi MMI 3G+ (HN+) Harman/Becker FLDB spatial indexing requirements.

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// WGS84 semi-major axis (equatorial radius) in meters (EPSG:4326 / EPSG:3857).
pub const WGS84_A: f64 = 6_378_137.0;

/// Maximum latitude for Web Mercator projection (clamped to prevent infinity at poles).
pub const WEB_MERCATOR_MAX_LAT: f64 = 85.05112878;

/// 32-bit signed fixed-point scaling factor: 2^31 - 1.
pub const FIXED_POINT_SCALE_I32: f64 = 2_147_483_647.0;

/// Error types for geographic transformations.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum GeoError {
    #[error("Latitude out of range [-90.0, 90.0]: {0}")]
    LatitudeOutOfRange(f64),
    #[error("Longitude out of range [-180.0, 180.0]: {0}")]
    LongitudeOutOfRange(f64),
    #[error("Validation error: {0}")]
    ValidationError(String),
}

/// Geographic point in WGS84 coordinates (degrees).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Wgs84Point {
    pub lat: f64,
    pub lon: f64,
    pub elevation_m: Option<i16>,
}

impl Wgs84Point {
    /// Creates a new validated Wgs84Point.
    pub fn new(lat: f64, lon: f64) -> Result<Self, GeoError> {
        if !(-90.0..=90.0).contains(&lat) {
            return Err(GeoError::LatitudeOutOfRange(lat));
        }
        if !(-180.0..=180.0).contains(&lon) {
            return Err(GeoError::LongitudeOutOfRange(lon));
        }
        Ok(Self {
            lat,
            lon,
            elevation_m: None,
        })
    }

    /// Creates a point with elevation.
    pub fn with_elevation(lat: f64, lon: f64, elevation_m: i16) -> Result<Self, GeoError> {
        let mut pt = Self::new(lat, lon)?;
        pt.elevation_m = Some(elevation_m);
        Ok(pt)
    }

    /// Projects to Planar Web Mercator (EPSG:3857) in meters.
    pub fn to_web_mercator(&self) -> WebMercatorPoint {
        let clamped_lat = self.lat.clamp(-WEB_MERCATOR_MAX_LAT, WEB_MERCATOR_MAX_LAT);
        let lambda_rad = self.lon.to_radians();
        let phi_rad = clamped_lat.to_radians();

        let x = WGS84_A * lambda_rad;
        let y = WGS84_A * (PI / 4.0 + phi_rad / 2.0).tan().ln();

        WebMercatorPoint { x, y }
    }

    /// Converts WGS84 coordinate to Harman/Becker 32-bit signed fixed-point integers.
    ///
    /// Longitude maps [-180.0, 180.0] -> [-2147483647, 2147483647]
    /// Latitude maps  [-90.0, 90.0]   -> [-2147483647, 2147483647]
    /// Provides sub-centimeter (<1 cm) precision everywhere in the ECE territory.
    pub fn to_fixed_point_32(&self) -> FixedPoint32 {
        let x_norm = (self.lon / 180.0).clamp(-1.0, 1.0);
        let y_norm = (self.lat / 90.0).clamp(-1.0, 1.0);

        let x_coord = (x_norm * FIXED_POINT_SCALE_I32).round() as i32;
        let y_coord = (y_norm * FIXED_POINT_SCALE_I32).round() as i32;

        FixedPoint32 { x_coord, y_coord }
    }

    /// Great-circle Haversine distance in meters to another point.
    pub fn haversine_distance_m(&self, other: &Wgs84Point) -> f64 {
        let d_lat = (other.lat - self.lat).to_radians();
        let d_lon = (other.lon - self.lon).to_radians();
        let lat1 = self.lat.to_radians();
        let lat2 = other.lat.to_radians();

        let a = (d_lat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (d_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        WGS84_A * c
    }

    /// Distance in decimeters (0.1 m resolution), matching the MMI `IrEdge.length_dm` specification.
    pub fn distance_dm(&self, other: &Wgs84Point) -> u32 {
        let dist_m = self.haversine_distance_m(other);
        (dist_m * 10.0).round().clamp(0.0, u32::MAX as f64) as u32
    }
}

/// Planar Web Mercator coordinate (EPSG:3857) in meters.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WebMercatorPoint {
    pub x: f64,
    pub y: f64,
}

impl WebMercatorPoint {
    /// Inverts Planar Web Mercator coordinates back to WGS84 (degrees).
    pub fn to_wgs84(&self) -> Wgs84Point {
        let lon = (self.x / WGS84_A).to_degrees();
        let lat = (2.0 * (self.y / WGS84_A).exp().atan() - PI / 2.0).to_degrees();
        Wgs84Point {
            lat: lat.clamp(-90.0, 90.0),
            lon: lon.clamp(-180.0, 180.0),
            elevation_m: None,
        }
    }
}

/// 32-bit signed fixed-point integer coordinate space for Harman/Becker FLDB.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FixedPoint32 {
    pub x_coord: i32,
    pub y_coord: i32,
}

impl FixedPoint32 {
    /// Constructs from raw coordinates.
    pub const fn new(x_coord: i32, y_coord: i32) -> Self {
        Self { x_coord, y_coord }
    }

    /// Converts back to WGS84 degrees.
    pub fn to_wgs84(&self) -> (f64, f64) {
        let lon = (self.x_coord as f64 / FIXED_POINT_SCALE_I32) * 180.0;
        let lat = (self.y_coord as f64 / FIXED_POINT_SCALE_I32) * 90.0;
        (lat, lon)
    }

    /// Generates 64-bit Morton Z-order curve key by bit-interleaving X and Y.
    ///
    /// Offsets signed coordinates by 2^31 to map `[-2^31, 2^31 - 1]` to `[0, 2^32 - 1]`,
    /// then interleaves even bits from X and odd bits from Y.
    pub fn morton_key(&self) -> u64 {
        let ux = (self.x_coord as i64 + 2_147_483_648) as u32;
        let uy = (self.y_coord as i64 + 2_147_483_648) as u32;
        interleave_bits_32(ux, uy)
    }

    /// Reconstructs FixedPoint32 from a 64-bit Morton Z-order key.
    pub fn from_morton_key(key: u64) -> Self {
        let (ux, uy) = deinterleave_bits_64(key);
        let x_coord = (ux as i64 - 2_147_483_648) as i32;
        let y_coord = (uy as i64 - 2_147_483_648) as i32;
        Self { x_coord, y_coord }
    }
}

/// Spreads 32 bits into 64 bits with zeroes in odd bit positions.
#[inline]
fn spread_bits_32(v: u32) -> u64 {
    let mut x = v as u64;
    x = (x | (x << 16)) & 0x0000_FFFF_0000_FFFF;
    x = (x | (x << 8))  & 0x00FF_00FF_00FF_00FF;
    x = (x | (x << 4))  & 0x0F0F_0F0F_0F0F_0F0F;
    x = (x | (x << 2))  & 0x3333_3333_3333_3333;
    x = (x | (x << 1))  & 0x5555_5555_5555_5555;
    x
}

/// Compacts 64 bits (with valid bits at even positions) back to 32 bits.
#[inline]
fn compact_bits_64(mut x: u64) -> u32 {
    x &= 0x5555_5555_5555_5555;
    x = (x | (x >> 1)) & 0x3333_3333_3333_3333;
    x = (x | (x >> 2)) & 0x0F0F_0F0F_0F0F_0F0F;
    x = (x | (x >> 4)) & 0x00FF_00FF_00FF_00FF;
    x = (x | (x >> 8)) & 0x0000_FFFF_0000_FFFF;
    x = (x | (x >> 16)) & 0x0000_0000_FFFF_FFFF;
    x as u32
}

/// Interleaves two 32-bit unsigned integers into a 64-bit Morton code.
/// Even bits come from `x`, odd bits come from `y`.
#[inline]
pub fn interleave_bits_32(x: u32, y: u32) -> u64 {
    spread_bits_32(x) | (spread_bits_32(y) << 1)
}

/// De-interleaves a 64-bit Morton code back into `(x, y)` 32-bit integers.
#[inline]
pub fn deinterleave_bits_64(morton: u64) -> (u32, u32) {
    let x = compact_bits_64(morton);
    let y = compact_bits_64(morton >> 1);
    (x, y)
}

/// Geographic bounding box in WGS84 degrees.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BoundingBox {
    pub min_lat: f64,
    pub min_lon: f64,
    pub max_lat: f64,
    pub max_lon: f64,
}

impl BoundingBox {
    /// Creates a new bounding box.
    #[inline]
    pub const fn new(min_lat: f64, min_lon: f64, max_lat: f64, max_lon: f64) -> Self {
        Self {
            min_lat,
            min_lon,
            max_lat,
            max_lon,
        }
    }

    /// Computes the geographic center (lat, lon) of the bounding box.
    #[inline]
    pub fn center(&self) -> (f64, f64) {
        ((self.min_lat + self.max_lat) * 0.5, (self.min_lon + self.max_lon) * 0.5)
    }

    /// Expands the bounding box to enclose the given point.
    #[inline]
    pub fn expand_point(&mut self, lat: f64, lon: f64) {
        if lat < self.min_lat {
            self.min_lat = lat;
        }
        if lat > self.max_lat {
            self.max_lat = lat;
        }
        if lon < self.min_lon {
            self.min_lon = lon;
        }
        if lon > self.max_lon {
            self.max_lon = lon;
        }
    }

    /// Checks if point (lat, lon) is contained in this bounding box.
    #[inline]
    pub fn contains_point(&self, lat: f64, lon: f64) -> bool {
        lat >= self.min_lat && lat <= self.max_lat && lon >= self.min_lon && lon <= self.max_lon
    }

    /// Checks if another bounding box intersects with this one.
    #[inline]
    pub fn intersects(&self, other: &BoundingBox) -> bool {
        self.min_lat <= other.max_lat
            && self.max_lat >= other.min_lat
            && self.min_lon <= other.max_lon
            && self.max_lon >= other.min_lon
    }
}

/// 3-Tier Regional Profiles for compilation and filtering.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegionalProfile {
    pub code: String,
    pub name: String,
    pub bbox: BoundingBox,
    pub estimated_nodes: u64,
    pub estimated_edges: u64,
    pub max_volumes: usize,
    pub target_size_bytes: u64,
}

impl RegionalProfile {
    /// Micro Albania & Western Balkans profile (~50 MB, single volume, <10s compile).
    pub fn micro_albania() -> Self {
        Self {
            code: "AL".to_string(),
            name: "Albania & Western Balkans".to_string(),
            bbox: BoundingBox::new(39.5, 19.0, 42.8, 21.2),
            estimated_nodes: 150_000,
            estimated_edges: 200_000,
            max_volumes: 1,
            target_size_bytes: 52_428_800, // ~50 MB
        }
    }

    /// Western Balkans Transit Corridor profile (~400 MB, single volume).
    pub fn western_balkans() -> Self {
        Self {
            code: "BALKANS".to_string(),
            name: "Western Balkans Transit Corridor".to_string(),
            bbox: BoundingBox::new(38.5, 18.0, 44.5, 23.5),
            estimated_nodes: 1_200_000,
            estimated_edges: 1_600_000,
            max_volumes: 1,
            target_size_bytes: 419_430_400, // ~400 MB
        }
    }

    /// Regional DACH profile (Germany, Austria, Switzerland, ~6.2 GB, 3 volumes).
    pub fn regional_dach() -> Self {
        Self {
            code: "DACH".to_string(),
            name: "DACH (Germany, Austria, Switzerland)".to_string(),
            bbox: BoundingBox::new(45.8, 5.8, 55.1, 17.2),
            estimated_nodes: 8_000_000,
            estimated_edges: 11_000_000,
            max_volumes: 3,
            target_size_bytes: 6_657_199_308, // ~6.2 GB
        }
    }

    /// Continental ECE profile (Full European territory, ~28.2 GB, 23 volumes).
    pub fn continental_ece() -> Self {
        Self {
            code: "ECE".to_string(),
            name: "Full European Territory (ECE)".to_string(),
            bbox: BoundingBox::new(34.5, -25.0, 71.5, 45.0),
            estimated_nodes: 50_000_000,
            estimated_edges: 70_000_000,
            max_volumes: 23,
            target_size_bytes: 28_185_247_890, // ~28.19 GB genuine MMI3GP footprint
        }
    }

    /// Finds a regional profile by code ("AL", "BALKANS", "DACH", "ECE" and aliases).
    pub fn from_code(code: &str) -> Option<Self> {
        match code.to_uppercase().as_str() {
            "AL" | "WB" | "MICRO" | "AL_CORRIDOR" | "AL_CORRIDOR_2026" => Some(Self::micro_albania()),
            "BALKANS" | "BALKANS_TRANSIT" | "BALKANS_TRANSIT_2026" => Some(Self::western_balkans()),
            "DACH" | "REGIONAL" | "DACH_REGIONAL" => Some(Self::regional_dach()),
            "ECE" | "EU" | "CONTINENTAL" | "ECE_FULL" | "ECE_FULL_2026" => Some(Self::continental_ece()),
            _ => None,
        }
    }

    /// Returns all standard regional profiles.
    pub fn all_profiles() -> Vec<Self> {
        vec![
            Self::micro_albania(),
            Self::western_balkans(),
            Self::regional_dach(),
            Self::continental_ece(),
        ]
    }
}

// -----------------------------------------------------------------------------
// Intermediate Representation (IR) Data Models
// Interface contract between Ingestion (M1) and FLDB Compiler (M2)
// -----------------------------------------------------------------------------

/// Junction flags for `IrNode`.
pub mod junction_flags {
    pub const NONE: u8 = 0x00;
    pub const ROUNDABOUT: u8 = 0x01;
    pub const TRAFFIC_SIGNAL: u8 = 0x02;
    pub const MOTORWAY_JUNCTION: u8 = 0x04;
    pub const BORDER_CROSSING: u8 = 0x08;
}

/// Access flags for `IrEdge`.
pub mod access_flags {
    pub const NONE: u8 = 0x00;
    pub const TOLL: u8 = 0x01;
    pub const TUNNEL: u8 = 0x02;
    pub const BRIDGE: u8 = 0x04;
    pub const FERRY: u8 = 0x08;
    pub const MOTOR_VEHICLE: u8 = 0x10;
}

/// Compact 3D Geographic Routing Node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IrNode {
    pub node_id: u64,
    pub x_coord: i32,
    pub y_coord: i32,
    pub elevation_m: i16,
    pub junction_flags: u8,
    pub edge_count: u8,
}

impl IrNode {
    /// Creates a new IrNode from WGS84 coordinates.
    pub fn from_wgs84(node_id: u64, lat: f64, lon: f64, elevation_m: i16, flags: u8) -> Self {
        let fp = Wgs84Point::new(lat, lon)
            .unwrap_or(Wgs84Point { lat: 0.0, lon: 0.0, elevation_m: None })
            .to_fixed_point_32();
        Self {
            node_id,
            x_coord: fp.x_coord,
            y_coord: fp.y_coord,
            elevation_m,
            junction_flags: flags,
            edge_count: 0,
        }
    }

    /// Converts coordinates back to WGS84 `(lat, lon)`.
    pub fn to_wgs84(&self) -> (f64, f64) {
        FixedPoint32::new(self.x_coord, self.y_coord).to_wgs84()
    }

    /// Morton Z-order curve index.
    pub fn morton_key(&self) -> u64 {
        FixedPoint32::new(self.x_coord, self.y_coord).morton_key()
    }
}

/// Directional Routing Edge (Link).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IrEdge {
    pub edge_id: u32,
    pub from_node: u32,
    pub to_node: u32,
    pub length_dm: u32,
    pub frc: u8,
    pub speed_forward: u8,
    pub speed_reverse: u8,
    pub lane_count: u8,
    pub turn_lane_mask: u16,
    pub geometry: Vec<(i32, i32)>,
    pub access_flags: u8,
}

/// Turn Restriction Rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IrTurnRestriction {
    pub from_edge: u32,
    pub via_node: u32,
    pub to_edge: u32,
    pub restriction_type: u8,
    pub penalty_s: u16,
}

/// Turn restriction types.
pub mod restriction_types {
    pub const NO_LEFT_TURN: u8 = 1;
    pub const NO_RIGHT_TURN: u8 = 2;
    pub const NO_U_TURN: u8 = 3;
    pub const NO_STRAIGHT_ON: u8 = 4;
    pub const ONLY_RIGHT_TURN: u8 = 5;
    pub const ONLY_LEFT_TURN: u8 = 6;
    pub const ONLY_STRAIGHT_ON: u8 = 7;
}

/// Commercial Point of Interest enriched via Google Maps Platform.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IrPoi {
    pub id: u32,
    pub name: String,
    pub category: String,
    pub lat: f64,
    pub lon: f64,
    pub x_mercator: i32,
    pub y_mercator: i32,
    pub brand: Option<String>,
    pub power_kw: Option<f32>,
    pub connectors: Vec<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub google_place_id: Option<String>,
}

impl IrPoi {
    /// Creates a new IrPoi, automatically populating Mercator fixed-point coordinates.
    pub fn new(
        id: u32,
        name: String,
        category: String,
        lat: f64,
        lon: f64,
        brand: Option<String>,
    ) -> Self {
        let pt = Wgs84Point::new(lat, lon).unwrap_or(Wgs84Point { lat: 0.0, lon: 0.0, elevation_m: None });
        let fp = pt.to_fixed_point_32();
        Self {
            id,
            name,
            category,
            lat,
            lon,
            x_mercator: fp.x_coord,
            y_mercator: fp.y_coord,
            brand,
            power_kw: None,
            connectors: Vec::new(),
            address: None,
            phone: None,
            google_place_id: None,
        }
    }
}

/// Complete Intermediate Representation Dataset for M2 FLDB compiler ingestion.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IrDataset {
    pub nodes: Vec<IrNode>,
    pub edges: Vec<IrEdge>,
    pub restrictions: Vec<IrTurnRestriction>,
    pub pois: Vec<IrPoi>,
    pub bounding_box: BoundingBox,
    pub region_profile: Option<String>,
    pub created_at: u64,
}

impl IrDataset {
    /// Creates a new empty IrDataset.
    pub fn new(bounding_box: BoundingBox, region_profile: Option<String>) -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            restrictions: Vec::new(),
            pois: Vec::new(),
            bounding_box,
            region_profile,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
        }
    }

    /// Validates integrity of node references in edges and turn restrictions.
    pub fn validate(&self) -> Result<(), GeoError> {
        let node_count = self.nodes.len();
        let edge_count = self.edges.len();

        for edge in &self.edges {
            if (edge.from_node as usize) >= node_count {
                return Err(GeoError::ValidationError(format!(
                    "Edge {} references invalid from_node index {}",
                    edge.edge_id, edge.from_node
                )));
            }
            if (edge.to_node as usize) >= node_count {
                return Err(GeoError::ValidationError(format!(
                    "Edge {} references invalid to_node index {}",
                    edge.edge_id, edge.to_node
                )));
            }
        }

        for rest in &self.restrictions {
            if rest.from_edge as usize >= edge_count {
                return Err(GeoError::ValidationError(format!(
                    "Restriction references invalid from_edge {}",
                    rest.from_edge
                )));
            }
            if rest.to_edge as usize >= edge_count {
                return Err(GeoError::ValidationError(format!(
                    "Restriction references invalid to_edge {}",
                    rest.to_edge
                )));
            }
        }

        Ok(())
    }

    /// Serializes dataset to JSON string.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Deserializes dataset from JSON string.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Generates summary metrics string.
    pub fn summary(&self) -> String {
        format!(
            "IrDataset: {} nodes, {} edges, {} restrictions, {} POIs (Region: {:?})",
            self.nodes.len(),
            self.edges.len(),
            self.restrictions.len(),
            self.pois.len(),
            self.region_profile
        )
    }
}
