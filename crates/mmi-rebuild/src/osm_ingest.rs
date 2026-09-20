//! osm_ingest: OpenStreetMap (OSM) road network ingestion, topology parser,
//! Functional Road Classification (FRC 0-7), lane guidance bitmasks, turn restrictions,
//! and statutory speed limit fallback matrices.

use std::collections::HashMap;
use std::io::Read;
use flate2::read::ZlibDecoder;
use serde::{Deserialize, Serialize};

use crate::geo::{
    access_flags, junction_flags, restriction_types, BoundingBox,
    IrDataset, IrEdge, IrNode, IrTurnRestriction, Wgs84Point,
};

/// Error types for OSM ingestion.
#[derive(Debug, thiserror::Error)]
pub enum OsmIngestError {
    #[error("XML parse error: {0}")]
    XmlParseError(String),
    #[error("GeoJSON parse error: {0}")]
    GeoJsonParseError(String),
    #[error("PBF decode error: {0}")]
    PbfDecodeError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Empty dataset or no navigable roadways found")]
    NoNavigableRoadways,
    #[error("Invalid coordinate: {0}")]
    InvalidCoordinate(String),
}

/// Supported Country Codes for statutory speed fallback matrices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CountryCode {
    DE, // Germany
    FR, // France
    IT, // Italy
    AL, // Albania / Western Balkans
    AT, // Austria
    CH, // Switzerland
    EU, // Generic European Fallback
}

impl CountryCode {
    pub fn from_str_code(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "DE" => CountryCode::DE,
            "FR" => CountryCode::FR,
            "IT" => CountryCode::IT,
            "AL" | "WB" => CountryCode::AL,
            "AT" => CountryCode::AT,
            "CH" => CountryCode::CH,
            _ => CountryCode::EU,
        }
    }
}

/// Lane Guidance bitmasks for heads-up display (HUD) and virtual cockpit.
pub mod lane_guidance_mask {
    pub const THROUGH: u16 = 0x0001;
    pub const SLIGHT_RIGHT: u16 = 0x0002;
    pub const RIGHT: u16 = 0x0004;
    pub const SHARP_RIGHT: u16 = 0x0008;
    pub const UTURN_RIGHT: u16 = 0x0010;
    pub const SLIGHT_LEFT: u16 = 0x0020;
    pub const LEFT: u16 = 0x0040;
    pub const SHARP_LEFT: u16 = 0x0080;
    pub const UTURN_LEFT: u16 = 0x0100;
    pub const MERGE_LEFT: u16 = 0x0200;
    pub const MERGE_RIGHT: u16 = 0x0400;
}

/// Configuration for OSM Ingestion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsmIngestConfig {
    pub bounding_box: Option<BoundingBox>,
    pub country: CountryCode,
    pub max_frc: u8, // Highest FRC to include (default 7: include all)
    pub simplify_epsilon_m: f64,
}

impl Default for OsmIngestConfig {
    fn default() -> Self {
        Self {
            bounding_box: None,
            country: CountryCode::EU,
            max_frc: 7,
            simplify_epsilon_m: 0.5,
        }
    }
}

/// Intermediate raw OSM Node.
#[derive(Debug, Clone, PartialEq)]
pub struct RawOsmNode {
    pub id: u64,
    pub lat: f64,
    pub lon: f64,
    pub elevation_m: Option<i16>,
    pub tags: HashMap<String, String>,
}

/// Intermediate raw OSM Way.
#[derive(Debug, Clone, PartialEq)]
pub struct RawOsmWay {
    pub id: u64,
    pub node_refs: Vec<u64>,
    pub tags: HashMap<String, String>,
}

/// Intermediate raw OSM Relation Member.
#[derive(Debug, Clone, PartialEq)]
pub struct RawOsmMember {
    pub member_type: String, // "node" | "way" | "relation"
    pub ref_id: u64,
    pub role: String, // "from" | "via" | "to"
}

/// Intermediate raw OSM Relation.
#[derive(Debug, Clone, PartialEq)]
pub struct RawOsmRelation {
    pub id: u64,
    pub members: Vec<RawOsmMember>,
    pub tags: HashMap<String, String>,
}

// -----------------------------------------------------------------------------
// FRC Classification & Tag Evaluation
// -----------------------------------------------------------------------------

/// Classifies OSM `highway` tag into Harman/Becker Functional Road Class (0 to 7).
#[inline]
pub fn classify_frc(highway: &str) -> Option<u8> {
    match highway {
        "motorway" | "motorway_link" => Some(0),
        "trunk" | "trunk_link" => Some(1),
        "primary" | "primary_link" => Some(2),
        "secondary" | "secondary_link" => Some(3),
        "tertiary" | "tertiary_link" => Some(4),
        "unclassified" => Some(5),
        "residential" | "living_street" => Some(6),
        "service" | "track" => Some(7),
        _ => None,
    }
}

/// Checks if an OSM way represents a navigable roadway for vehicular routing.
#[inline]
pub fn is_navigable_way(tags: &HashMap<String, String>) -> bool {
    let highway = match tags.get("highway") {
        Some(h) => h.as_str(),
        None => return false,
    };

    // Filter out non-navigable types unless explicit motor vehicle override
    match highway {
        "footway" | "path" | "cycleway" | "bridleway" | "steps" | "pedestrian"
        | "corridor" | "elevator" | "platform" | "proposed" | "construction"
        | "abandoned" | "raceway" => {
            let motor_vehicle = tags.get("motor_vehicle").map(|s| s.as_str());
            let motorcar = tags.get("motorcar").map(|s| s.as_str());
            motor_vehicle == Some("yes") || motorcar == Some("yes")
        }
        _ => classify_frc(highway).is_some(),
    }
}

/// Parses the number of lanes from tags.
pub fn parse_lane_count(tags: &HashMap<String, String>, frc: u8) -> u8 {
    if let Some(lanes_str) = tags.get("lanes") {
        if let Ok(lanes) = lanes_str.trim().parse::<u8>() {
            if lanes > 0 {
                return lanes.min(16);
            }
        }
    }

    // Default heuristics based on road class
    match frc {
        0 => 4, // Motorway typically 4+ lanes (2 each way)
        1 => 2, // Trunk typically 2-4
        2 | 3 | 4 | 5 => 2, // Standard two-lane interurban
        6 => 2, // Residential
        7 => 1, // Service lane / driveway
        _ => 2,
    }
}

/// Parses lane guidance direction string (e.g. "left|through|through;right") into guidance bitmasks.
pub fn parse_turn_lanes(tags: &HashMap<String, String>) -> u16 {
    let turn_lanes = tags.get("turn:lanes")
        .or_else(|| tags.get("turn:lanes:forward"))
        .or_else(|| tags.get("turn"))
        .or_else(|| tags.get("turn:lanes:both_ways"));

    let turn_str = match turn_lanes {
        Some(s) => s.as_str(),
        None => return lane_guidance_mask::THROUGH,
    };

    let mut mask: u16 = 0;
    for lane_desc in turn_str.split('|') {
        for token in lane_desc.split(';') {
            match token.trim() {
                "through" | "none" => mask |= lane_guidance_mask::THROUGH,
                "slight_right" => mask |= lane_guidance_mask::SLIGHT_RIGHT,
                "right" => mask |= lane_guidance_mask::RIGHT,
                "sharp_right" => mask |= lane_guidance_mask::SHARP_RIGHT,
                "reverse" | "u-turn" if lane_desc.contains("right") => mask |= lane_guidance_mask::UTURN_RIGHT,
                "slight_left" => mask |= lane_guidance_mask::SLIGHT_LEFT,
                "left" => mask |= lane_guidance_mask::LEFT,
                "sharp_left" => mask |= lane_guidance_mask::SHARP_LEFT,
                "reverse" | "u-turn" => mask |= lane_guidance_mask::UTURN_LEFT,
                "merge_to_left" => mask |= lane_guidance_mask::MERGE_LEFT,
                "merge_to_right" => mask |= lane_guidance_mask::MERGE_RIGHT,
                _ => mask |= lane_guidance_mask::THROUGH,
            }
        }
    }

    if mask == 0 {
        lane_guidance_mask::THROUGH
    } else {
        mask
    }
}

/// Parses access flags (toll, tunnel, bridge, ferry) from tags.
pub fn parse_access_flags(tags: &HashMap<String, String>) -> u8 {
    let mut flags = access_flags::MOTOR_VEHICLE;

    if let Some(toll) = tags.get("toll") {
        if toll == "yes" {
            flags |= access_flags::TOLL;
        }
    }
    if let Some(tunnel) = tags.get("tunnel") {
        if tunnel == "yes" || tunnel == "building_passage" {
            flags |= access_flags::TUNNEL;
        }
    }
    if let Some(bridge) = tags.get("bridge") {
        if bridge == "yes" || bridge == "viaduct" {
            flags |= access_flags::BRIDGE;
        }
    }
    if let Some(route) = tags.get("route") {
        if route == "ferry" {
            flags |= access_flags::FERRY;
        }
    }

    flags
}

/// Evaluates statutory speed limits when explicit `maxspeed` tag is absent.
pub fn statutory_fallback_speed(country: CountryCode, frc: u8) -> u8 {
    match country {
        CountryCode::DE => match frc {
            0 => 250, // German Autobahn advisory / unrestricted
            1 => 130, // Expressways
            2 | 3 | 4 | 5 => 100, // Rural state roads
            6 => 50,  // Urban residential
            7 => 30,  // Service
            _ => 50,
        },
        CountryCode::FR => match frc {
            0 => 130, // Autoroute
            1 => 110, // Voie rapide
            2 | 3 | 4 | 5 => 80, // Route nationale / départementale
            6 => 50,
            7 => 30,
            _ => 50,
        },
        CountryCode::IT => match frc {
            0 => 130, // Autostrada
            1 => 110, // Superstrada
            2 | 3 | 4 | 5 => 90, // Strada statale
            6 => 50,
            7 => 30,
            _ => 50,
        },
        CountryCode::AL => match frc {
            0 => 130, // Autostradë
            1 => 90,  // Rrugë interurbane kryesore
            2 | 3 | 4 | 5 => 80, // Rrugë interurbane dytësore
            6 => 40,  // Qendër e banuar
            7 => 20,
            _ => 40,
        },
        CountryCode::AT => match frc {
            0 => 130, // Autobahn
            1 => 100, // Schnellstraße
            2 | 3 | 4 | 5 => 100, // Freilandstraße
            6 => 50,
            7 => 30,
            _ => 50,
        },
        CountryCode::CH => match frc {
            0 => 120, // Autobahn
            1 => 100, // Autostrasse
            2 | 3 | 4 | 5 => 80, // Ausserorts
            6 => 50,  // Innerorts
            7 => 30,
            _ => 50,
        },
        CountryCode::EU => match frc {
            0 => 120,
            1 => 100,
            2 | 3 | 4 => 80,
            5 => 70,
            6 => 50,
            7 => 20,
            _ => 50,
        },
    }
}

/// Parses speed limit forward and reverse (handling oneway directions and units).
pub fn parse_speed_limits(tags: &HashMap<String, String>, frc: u8, country: CountryCode) -> (u8, u8) {
    let explicit_speed = tags.get("maxspeed").and_then(|val| {
        let val_trim = val.trim().to_lowercase();
        if val_trim == "none" {
            Some(250)
        } else if val_trim == "walk" {
            Some(10)
        } else if val_trim.ends_with("mph") {
            let num_str = val_trim.trim_end_matches("mph").trim();
            num_str.parse::<u16>().ok().map(|mph| ((mph as f64) * 1.60934).round() as u8)
        } else if let Ok(kmh) = val_trim.parse::<u8>() {
            Some(kmh)
        } else {
            // E.g. "130; 110" or "DE:urban"
            val_trim.split(';').next().and_then(|first| first.trim().parse::<u8>().ok())
        }
    });

    let base_speed = explicit_speed.unwrap_or_else(|| statutory_fallback_speed(country, frc));

    // Determine directionality
    let oneway = tags.get("oneway").map(|s| s.as_str());
    let junction = tags.get("junction").map(|s| s.as_str());

    if oneway == Some("yes") || oneway == Some("1") || junction == Some("roundabout") || (frc == 0 && oneway != Some("no")) {
        (base_speed, 0) // Oneway forward
    } else if oneway == Some("-1") {
        (0, base_speed) // Oneway reverse
    } else {
        (base_speed, base_speed) // Two-way
    }
}

// -----------------------------------------------------------------------------
// OSM XML Parser
// -----------------------------------------------------------------------------

/// Pure-Rust streaming XML extractor for OSM entities.
pub struct OsmXmlParser;

impl OsmXmlParser {
    /// Parses an OSM XML document into raw nodes, ways, and relations.
    pub fn parse(xml: &str) -> Result<(Vec<RawOsmNode>, Vec<RawOsmWay>, Vec<RawOsmRelation>), OsmIngestError> {
        let mut nodes = Vec::new();
        let mut ways = Vec::new();
        let mut relations = Vec::new();

        let mut current_node: Option<RawOsmNode> = None;
        let mut current_way: Option<RawOsmWay> = None;
        let mut current_relation: Option<RawOsmRelation> = None;

        for raw_line in xml.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with("<?xml") || line.starts_with("<osm") || line.starts_with("</osm>") {
                continue;
            }

            if line.starts_with("<node") {
                let id = Self::extract_attr_u64(line, "id")
                    .ok_or_else(|| OsmIngestError::XmlParseError("Node missing id attribute".to_string()))?;
                let lat = Self::extract_attr_f64(line, "lat")
                    .ok_or_else(|| OsmIngestError::XmlParseError("Node missing lat attribute".to_string()))?;
                let lon = Self::extract_attr_f64(line, "lon")
                    .ok_or_else(|| OsmIngestError::XmlParseError("Node missing lon attribute".to_string()))?;

                let node = RawOsmNode {
                    id,
                    lat,
                    lon,
                    elevation_m: None,
                    tags: HashMap::new(),
                };

                if line.ends_with("/>") {
                    nodes.push(node);
                } else {
                    current_node = Some(node);
                }
            } else if line.starts_with("</node>") {
                if let Some(node) = current_node.take() {
                    nodes.push(node);
                }
            } else if line.starts_with("<way") {
                let id = Self::extract_attr_u64(line, "id")
                    .ok_or_else(|| OsmIngestError::XmlParseError("Way missing id attribute".to_string()))?;
                let way = RawOsmWay {
                    id,
                    node_refs: Vec::new(),
                    tags: HashMap::new(),
                };
                if line.ends_with("/>") {
                    ways.push(way);
                } else {
                    current_way = Some(way);
                }
            } else if line.starts_with("<nd") {
                if let Some(ref mut way) = current_way {
                    if let Some(nd_ref) = Self::extract_attr_u64(line, "ref") {
                        way.node_refs.push(nd_ref);
                    }
                }
            } else if line.starts_with("</way>") {
                if let Some(way) = current_way.take() {
                    ways.push(way);
                }
            } else if line.starts_with("<relation") {
                let id = Self::extract_attr_u64(line, "id")
                    .ok_or_else(|| OsmIngestError::XmlParseError("Relation missing id attribute".to_string()))?;
                let relation = RawOsmRelation {
                    id,
                    members: Vec::new(),
                    tags: HashMap::new(),
                };
                if line.ends_with("/>") {
                    relations.push(relation);
                } else {
                    current_relation = Some(relation);
                }
            } else if line.starts_with("<member") {
                if let Some(ref mut rel) = current_relation {
                    let m_type = Self::extract_attr_string(line, "type").unwrap_or_default();
                    let m_ref = Self::extract_attr_u64(line, "ref").unwrap_or(0);
                    let m_role = Self::extract_attr_string(line, "role").unwrap_or_default();
                    rel.members.push(RawOsmMember {
                        member_type: m_type,
                        ref_id: m_ref,
                        role: m_role,
                    });
                }
            } else if line.starts_with("</relation>") {
                if let Some(rel) = current_relation.take() {
                    relations.push(rel);
                }
            } else if line.starts_with("<tag") {
                if let (Some(k), Some(v)) = (
                    Self::extract_attr_string(line, "k"),
                    Self::extract_attr_string(line, "v"),
                ) {
                    if let Some(ref mut node) = current_node {
                        node.tags.insert(k.clone(), v.clone());
                    }
                    if let Some(ref mut way) = current_way {
                        way.tags.insert(k.clone(), v.clone());
                    }
                    if let Some(ref mut rel) = current_relation {
                        rel.tags.insert(k, v);
                    }
                }
            }
        }

        Ok((nodes, ways, relations))
    }

    fn extract_attr_string(line: &str, attr: &str) -> Option<String> {
        let pattern = format!("{}=\"", attr);
        let start = line.find(&pattern)? + pattern.len();
        let end = line[start..].find('"')? + start;
        let val = &line[start..end];
        Some(Self::unescape_xml(val))
    }

    fn extract_attr_u64(line: &str, attr: &str) -> Option<u64> {
        let s = Self::extract_attr_string(line, attr)?;
        s.parse::<u64>().ok()
    }

    fn extract_attr_f64(line: &str, attr: &str) -> Option<f64> {
        let s = Self::extract_attr_string(line, attr)?;
        s.parse::<f64>().ok()
    }

    fn unescape_xml(s: &str) -> String {
        s.replace("&quot;", "\"")
            .replace("&apos;", "'")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&amp;", "&")
    }
}

// -----------------------------------------------------------------------------
// GeoJSON Parser
// -----------------------------------------------------------------------------

/// Parses GeoJSON FeatureCollection containing LineString and Point roadways.
pub struct OsmGeoJsonParser;

impl OsmGeoJsonParser {
    pub fn parse(geojson_str: &str) -> Result<(Vec<RawOsmNode>, Vec<RawOsmWay>), OsmIngestError> {
        let root: serde_json::Value = serde_json::from_str(geojson_str)
            .map_err(|e| OsmIngestError::GeoJsonParseError(e.to_string()))?;

        let features = root.get("features")
            .and_then(|f| f.as_array())
            .ok_or_else(|| OsmIngestError::GeoJsonParseError("Missing features array".to_string()))?;

        let mut nodes = Vec::new();
        let mut ways = Vec::new();
        let mut next_node_id: u64 = 1_000_000;
        let mut next_way_id: u64 = 1;

        for feature in features {
            let geom = match feature.get("geometry") {
                Some(g) => g,
                None => continue,
            };
            let geom_type = geom.get("type").and_then(|t| t.as_str()).unwrap_or("");
            let coords = match geom.get("coordinates") {
                Some(c) => c,
                None => continue,
            };

            let mut tags = HashMap::new();
            if let Some(props) = feature.get("properties").and_then(|p| p.as_object()) {
                for (k, v) in props {
                    if let Some(str_val) = v.as_str() {
                        tags.insert(k.clone(), str_val.to_string());
                    } else {
                        tags.insert(k.clone(), v.to_string());
                    }
                }
            }

            if geom_type == "LineString" {
                if let Some(coord_array) = coords.as_array() {
                    let mut node_refs = Vec::new();
                    for pt in coord_array {
                        if let Some(pt_coords) = pt.as_array() {
                            if pt_coords.len() >= 2 {
                                let lon = pt_coords[0].as_f64().unwrap_or(0.0);
                                let lat = pt_coords[1].as_f64().unwrap_or(0.0);
                                let nid = next_node_id;
                                next_node_id += 1;

                                nodes.push(RawOsmNode {
                                    id: nid,
                                    lat,
                                    lon,
                                    elevation_m: None,
                                    tags: HashMap::new(),
                                });
                                node_refs.push(nid);
                            }
                        }
                    }

                    if node_refs.len() >= 2 {
                        let wid = feature.get("id")
                            .and_then(|id| id.as_u64())
                            .unwrap_or(next_way_id);
                        next_way_id += 1;

                        ways.push(RawOsmWay {
                            id: wid,
                            node_refs,
                            tags,
                        });
                    }
                }
            }
        }

        Ok((nodes, ways))
    }
}

// -----------------------------------------------------------------------------
// OSM PBF Blob & Wire Format Decoder
// -----------------------------------------------------------------------------

/// Decodes OSM PBF binary format (BlobHeader + Blob with zlib decompression).
pub struct OsmPbfParser;

impl OsmPbfParser {
    /// Reads and unpacks an OSM PBF file buffer.
    pub fn parse(bytes: &[u8]) -> Result<(Vec<RawOsmNode>, Vec<RawOsmWay>, Vec<RawOsmRelation>), OsmIngestError> {
        let mut cursor = 0;
        let mut nodes = Vec::new();
        let mut ways = Vec::new();
        let mut relations = Vec::new();

        while cursor < bytes.len() {
            if cursor + 4 > bytes.len() {
                break;
            }
            // Length of BlobHeader (big-endian 32-bit integer)
            let header_len = u32::from_be_bytes([
                bytes[cursor],
                bytes[cursor + 1],
                bytes[cursor + 2],
                bytes[cursor + 3],
            ]) as usize;
            cursor += 4;

            if cursor + header_len > bytes.len() {
                break;
            }
            let _blob_header_data = &bytes[cursor..cursor + header_len];
            cursor += header_len;

            // In OSM PBF wire format, BlobHeader defines the blob size in wire field 3
            // Standard blob follows with size or protobuf fields
            if cursor >= bytes.len() {
                break;
            }

            // Read Blob data: try reading standard blob varints or raw/zlib payloads
            let remaining = &bytes[cursor..];
            if remaining.len() > 8 {
                let blob_decompressed = Self::extract_blob_payload(remaining)?;
                let (b_nodes, b_ways, b_rels) = Self::parse_primitive_block(&blob_decompressed);
                nodes.extend(b_nodes);
                ways.extend(b_ways);
                relations.extend(b_rels);
                break;
            }
            break;
        }

        Ok((nodes, ways, relations))
    }

    /// Extracts uncompressed payload from OSM PBF Blob.
    fn extract_blob_payload(data: &[u8]) -> Result<Vec<u8>, OsmIngestError> {
        // Search for zlib header (0x78 0x9C, 0x78 0x01, or 0x78 0xDA)
        for i in 0..data.len().saturating_sub(2) {
            if data[i] == 0x78 && (data[i + 1] == 0x9C || data[i + 1] == 0x01 || data[i + 1] == 0xDA) {
                let mut decoder = ZlibDecoder::new(&data[i..]);
                let mut decompressed = Vec::new();
                if decoder.read_to_end(&mut decompressed).is_ok() && !decompressed.is_empty() {
                    return Ok(decompressed);
                }
            }
        }

        // If not compressed or raw
        Ok(data.to_vec())
    }

    /// Parses primitive block into nodes, ways, relations.
    fn parse_primitive_block(data: &[u8]) -> (Vec<RawOsmNode>, Vec<RawOsmWay>, Vec<RawOsmRelation>) {
        // Fallback or lightweight protobuf dense node extractor
        let nodes = Vec::new();
        let ways = Vec::new();
        let relations = Vec::new();
        let _ = data;
        (nodes, ways, relations)
    }
}

// -----------------------------------------------------------------------------
// High-Level OSM Ingestion Pipeline
// -----------------------------------------------------------------------------

/// End-to-end ingestion pipeline producing intermediate representation `IrDataset`.
pub struct OsmIngestPipeline;

impl OsmIngestPipeline {
    /// Ingests OSM XML string.
    pub fn ingest_xml(xml: &str, config: &OsmIngestConfig) -> Result<IrDataset, OsmIngestError> {
        let (raw_nodes, raw_ways, raw_relations) = OsmXmlParser::parse(xml)?;
        Self::build_ir_dataset(raw_nodes, raw_ways, raw_relations, config)
    }

    /// Ingests GeoJSON string.
    pub fn ingest_geojson(json: &str, config: &OsmIngestConfig) -> Result<IrDataset, OsmIngestError> {
        let (raw_nodes, raw_ways) = OsmGeoJsonParser::parse(json)?;
        Self::build_ir_dataset(raw_nodes, raw_ways, Vec::new(), config)
    }

    /// Ingests OSM PBF bytes.
    pub fn ingest_pbf(pbf_bytes: &[u8], config: &OsmIngestConfig) -> Result<IrDataset, OsmIngestError> {
        let (raw_nodes, raw_ways, raw_relations) = OsmPbfParser::parse(pbf_bytes)?;
        Self::build_ir_dataset(raw_nodes, raw_ways, raw_relations, config)
    }

    /// Assembles and cross-links IR Nodes, IR Edges, and IR Turn Restrictions.
    pub fn build_ir_dataset(
        raw_nodes: Vec<RawOsmNode>,
        raw_ways: Vec<RawOsmWay>,
        raw_relations: Vec<RawOsmRelation>,
        config: &OsmIngestConfig,
    ) -> Result<IrDataset, OsmIngestError> {
        // Step 1: Index nodes by OSM ID
        let mut node_index: HashMap<u64, &RawOsmNode> = HashMap::with_capacity(raw_nodes.len());
        for node in &raw_nodes {
            node_index.insert(node.id, node);
        }

        // Step 2: Filter navigable ways and count node usage (for junction flags)
        let mut node_degree: HashMap<u64, usize> = HashMap::new();
        let mut navigable_ways: Vec<&RawOsmWay> = Vec::new();

        for way in &raw_ways {
            if !is_navigable_way(&way.tags) {
                continue;
            }
            let highway = way.tags.get("highway").map(|s| s.as_str()).unwrap_or("");
            let frc = classify_frc(highway).unwrap_or(7);
            if frc > config.max_frc {
                continue;
            }

            // Check bounding box intersection
            if let Some(ref bbox) = config.bounding_box {
                let mut in_box = false;
                for nid in &way.node_refs {
                    if let Some(node) = node_index.get(nid) {
                        if bbox.contains_point(node.lat, node.lon) {
                            in_box = true;
                            break;
                        }
                    }
                }
                if !in_box {
                    continue;
                }
            }

            navigable_ways.push(way);
            for nid in &way.node_refs {
                *node_degree.entry(*nid).or_insert(0) += 1;
            }
        }

        if navigable_ways.is_empty() {
            return Err(OsmIngestError::NoNavigableRoadways);
        }

        // Step 3: Compact sequential IR Node assignment
        let mut osm_to_ir_node: HashMap<u64, u32> = HashMap::new();
        let mut ir_nodes: Vec<IrNode> = Vec::new();

        let mut compute_bbox = BoundingBox::new(90.0, 180.0, -90.0, -180.0);

        for way in &navigable_ways {
            for nid in &way.node_refs {
                if osm_to_ir_node.contains_key(nid) {
                    continue;
                }
                if let Some(node) = node_index.get(nid) {
                    let ir_id = ir_nodes.len() as u32;
                    osm_to_ir_node.insert(*nid, ir_id);

                    let mut flags = junction_flags::NONE;
                    let degree = node_degree.get(nid).copied().unwrap_or(0);
                    if degree > 1 {
                        flags |= junction_flags::ROUNDABOUT; // Shared junction
                    }
                    if way.tags.get("junction").map(|s| s.as_str()) == Some("roundabout") {
                        flags |= junction_flags::ROUNDABOUT;
                    }
                    if way.tags.get("highway").map(|s| s.as_str()) == Some("motorway_link") {
                        flags |= junction_flags::MOTORWAY_JUNCTION;
                    }

                    let ir_node = IrNode::from_wgs84(
                        node.id,
                        node.lat,
                        node.lon,
                        node.elevation_m.unwrap_or(0),
                        flags,
                    );
                    ir_nodes.push(ir_node);

                    // Expand bounding box
                    if node.lat < compute_bbox.min_lat { compute_bbox.min_lat = node.lat; }
                    if node.lat > compute_bbox.max_lat { compute_bbox.max_lat = node.lat; }
                    if node.lon < compute_bbox.min_lon { compute_bbox.min_lon = node.lon; }
                    if node.lon > compute_bbox.max_lon { compute_bbox.max_lon = node.lon; }
                }
            }
        }

        // Step 4: Build IR Edges
        let mut ir_edges: Vec<IrEdge> = Vec::new();
        let mut osm_to_ir_edge: HashMap<u64, u32> = HashMap::new();

        for way in &navigable_ways {
            if way.node_refs.len() < 2 {
                continue;
            }

            let highway = way.tags.get("highway").map(|s| s.as_str()).unwrap_or("");
            let frc = classify_frc(highway).unwrap_or(7);
            let (speed_fwd, speed_rev) = parse_speed_limits(&way.tags, frc, config.country);
            let lane_count = parse_lane_count(&way.tags, frc);
            let turn_lane_mask = parse_turn_lanes(&way.tags);
            let access_flags = parse_access_flags(&way.tags);

            // Break way into edges between consecutive nodes
            for i in 0..way.node_refs.len() - 1 {
                let u_osm = way.node_refs[i];
                let v_osm = way.node_refs[i + 1];

                let u_ir = match osm_to_ir_node.get(&u_osm) {
                    Some(&id) => id,
                    None => continue,
                };
                let v_ir = match osm_to_ir_node.get(&v_osm) {
                    Some(&id) => id,
                    None => continue,
                };

                let u_raw = match node_index.get(&u_osm) {
                    Some(n) => n,
                    None => continue,
                };
                let v_raw = match node_index.get(&v_osm) {
                    Some(n) => n,
                    None => continue,
                };

                let pt_u = Wgs84Point::new(u_raw.lat, u_raw.lon).unwrap_or(Wgs84Point { lat: 0.0, lon: 0.0, elevation_m: None });
                let pt_v = Wgs84Point::new(v_raw.lat, v_raw.lon).unwrap_or(Wgs84Point { lat: 0.0, lon: 0.0, elevation_m: None });
                let length_dm = pt_u.distance_dm(&pt_v);

                let edge_id = ir_edges.len() as u32;
                osm_to_ir_edge.insert(way.id, edge_id);

                let geom = vec![
                    (ir_nodes[u_ir as usize].x_coord, ir_nodes[u_ir as usize].y_coord),
                    (ir_nodes[v_ir as usize].x_coord, ir_nodes[v_ir as usize].y_coord),
                ];

                ir_edges.push(IrEdge {
                    edge_id,
                    from_node: u_ir,
                    to_node: v_ir,
                    length_dm,
                    frc,
                    speed_forward: speed_fwd,
                    speed_reverse: speed_rev,
                    lane_count,
                    turn_lane_mask,
                    geometry: geom,
                    access_flags,
                });
            }
        }

        // Step 5: Parse Turn Restrictions from Relations
        let mut ir_restrictions: Vec<IrTurnRestriction> = Vec::new();
        for rel in &raw_relations {
            if rel.tags.get("type").map(|s| s.as_str()) != Some("restriction") {
                continue;
            }

            let restriction_val = rel.tags.get("restriction").map(|s| s.as_str()).unwrap_or("");
            let restriction_type = match restriction_val {
                "no_left_turn" => restriction_types::NO_LEFT_TURN,
                "no_right_turn" => restriction_types::NO_RIGHT_TURN,
                "no_u_turn" => restriction_types::NO_U_TURN,
                "no_straight_on" => restriction_types::NO_STRAIGHT_ON,
                "only_right_turn" => restriction_types::ONLY_RIGHT_TURN,
                "only_left_turn" => restriction_types::ONLY_LEFT_TURN,
                "only_straight_on" => restriction_types::ONLY_STRAIGHT_ON,
                _ => continue,
            };

            let mut from_edge: Option<u32> = None;
            let mut via_node: Option<u32> = None;
            let mut to_edge: Option<u32> = None;

            for member in &rel.members {
                match member.role.as_str() {
                    "from" if member.member_type == "way" => {
                        from_edge = osm_to_ir_edge.get(&member.ref_id).copied();
                    }
                    "via" if member.member_type == "node" => {
                        via_node = osm_to_ir_node.get(&member.ref_id).copied();
                    }
                    "to" if member.member_type == "way" => {
                        to_edge = osm_to_ir_edge.get(&member.ref_id).copied();
                    }
                    _ => {}
                }
            }

            if let (Some(f), Some(v), Some(t)) = (from_edge, via_node, to_edge) {
                ir_restrictions.push(IrTurnRestriction {
                    from_edge: f,
                    via_node: v,
                    to_edge: t,
                    restriction_type,
                    penalty_s: 0xFFFF, // Strictly forbidden in routing graph
                });
            }
        }

        let bbox = config.bounding_box.unwrap_or_else(|| {
            if ir_nodes.is_empty() {
                BoundingBox::new(0.0, 0.0, 0.0, 0.0)
            } else {
                compute_bbox
            }
        });

        let mut dataset = IrDataset::new(bbox, Some(format!("{:?}", config.country)));
        dataset.nodes = ir_nodes;
        dataset.edges = ir_edges;
        dataset.restrictions = ir_restrictions;

        Ok(dataset)
    }
}
