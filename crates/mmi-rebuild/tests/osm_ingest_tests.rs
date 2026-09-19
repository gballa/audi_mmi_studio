use std::collections::HashMap;
use std::fs;
use mmi_rebuild::geo::{access_flags, junction_flags, restriction_types};
use mmi_rebuild::osm_ingest::{
    classify_frc, is_navigable_way, lane_guidance_mask, parse_access_flags,
    parse_lane_count, parse_speed_limits, parse_turn_lanes, statutory_fallback_speed,
    CountryCode, OsmIngestConfig, OsmIngestPipeline,
};

#[test]
fn test_classify_frc_all_functional_classes() {
    assert_eq!(classify_frc("motorway"), Some(0));
    assert_eq!(classify_frc("motorway_link"), Some(0));
    assert_eq!(classify_frc("trunk"), Some(1));
    assert_eq!(classify_frc("trunk_link"), Some(1));
    assert_eq!(classify_frc("primary"), Some(2));
    assert_eq!(classify_frc("primary_link"), Some(2));
    assert_eq!(classify_frc("secondary"), Some(3));
    assert_eq!(classify_frc("secondary_link"), Some(3));
    assert_eq!(classify_frc("tertiary"), Some(4));
    assert_eq!(classify_frc("tertiary_link"), Some(4));
    assert_eq!(classify_frc("unclassified"), Some(5));
    assert_eq!(classify_frc("residential"), Some(6));
    assert_eq!(classify_frc("living_street"), Some(6));
    assert_eq!(classify_frc("service"), Some(7));
    assert_eq!(classify_frc("track"), Some(7));
    assert_eq!(classify_frc("footway"), None);
    assert_eq!(classify_frc("cycleway"), None);
}

#[test]
fn test_is_navigable_way() {
    let mut tags = HashMap::new();
    tags.insert("highway".to_string(), "primary".to_string());
    assert!(is_navigable_way(&tags));

    // Footway is non-navigable
    let mut foot_tags = HashMap::new();
    foot_tags.insert("highway".to_string(), "footway".to_string());
    assert!(!is_navigable_way(&foot_tags));

    // Footway with explicit motor_vehicle override is navigable
    foot_tags.insert("motor_vehicle".to_string(), "yes".to_string());
    assert!(is_navigable_way(&foot_tags));
}

#[test]
fn test_parse_lane_count() {
    let mut tags = HashMap::new();
    tags.insert("lanes".to_string(), "3".to_string());
    assert_eq!(parse_lane_count(&tags, 2), 3);

    // Default heuristics based on road class
    let empty_tags = HashMap::new();
    assert_eq!(parse_lane_count(&empty_tags, 0), 4); // Motorway default
    assert_eq!(parse_lane_count(&empty_tags, 2), 2); // Primary default
    assert_eq!(parse_lane_count(&empty_tags, 7), 1); // Service default
}

#[test]
fn test_parse_lane_guidance_bitmasks() {
    let mut tags = HashMap::new();
    tags.insert("turn:lanes".to_string(), "left|through;right".to_string());

    let mask = parse_turn_lanes(&tags);
    assert_ne!(mask & lane_guidance_mask::LEFT, 0);
    assert_ne!(mask & lane_guidance_mask::THROUGH, 0);
    assert_ne!(mask & lane_guidance_mask::RIGHT, 0);
    assert_eq!(mask & lane_guidance_mask::SHARP_LEFT, 0);

    // Test complex multiple arrows
    let mut complex_tags = HashMap::new();
    complex_tags.insert(
        "turn:lanes".to_string(),
        "slight_left|sharp_right|reverse".to_string(),
    );
    let c_mask = parse_turn_lanes(&complex_tags);
    assert_ne!(c_mask & lane_guidance_mask::SLIGHT_LEFT, 0);
    assert_ne!(c_mask & lane_guidance_mask::SHARP_RIGHT, 0);
    assert_ne!(c_mask & lane_guidance_mask::UTURN_LEFT, 0);
}

#[test]
fn test_statutory_speed_limit_fallbacks() {
    // Germany
    assert_eq!(statutory_fallback_speed(CountryCode::DE, 0), 250); // Autobahn
    assert_eq!(statutory_fallback_speed(CountryCode::DE, 2), 100); // Rural primary
    assert_eq!(statutory_fallback_speed(CountryCode::DE, 6), 50);  // Urban

    // France
    assert_eq!(statutory_fallback_speed(CountryCode::FR, 0), 130);
    assert_eq!(statutory_fallback_speed(CountryCode::FR, 1), 110);
    assert_eq!(statutory_fallback_speed(CountryCode::FR, 2), 80);

    // Albania
    assert_eq!(statutory_fallback_speed(CountryCode::AL, 0), 130);
    assert_eq!(statutory_fallback_speed(CountryCode::AL, 1), 90);
    assert_eq!(statutory_fallback_speed(CountryCode::AL, 2), 80);
    assert_eq!(statutory_fallback_speed(CountryCode::AL, 6), 40);

    // Austria
    assert_eq!(statutory_fallback_speed(CountryCode::AT, 0), 130);
    assert_eq!(statutory_fallback_speed(CountryCode::AT, 1), 100);

    // Switzerland
    assert_eq!(statutory_fallback_speed(CountryCode::CH, 0), 120);
    assert_eq!(statutory_fallback_speed(CountryCode::CH, 2), 80);
}

#[test]
fn test_parse_speed_limits_explicit_and_mph() {
    let mut tags = HashMap::new();
    tags.insert("maxspeed".to_string(), "60 mph".to_string());
    let (fwd, rev) = parse_speed_limits(&tags, 2, CountryCode::EU);
    assert_eq!(fwd, 97); // 60 mph = 96.56 km/h -> 97 km/h
    assert_eq!(rev, 97);

    // German Autobahn explicit "none"
    let mut de_tags = HashMap::new();
    de_tags.insert("maxspeed".to_string(), "none".to_string());
    let (de_fwd, de_rev) = parse_speed_limits(&de_tags, 0, CountryCode::DE);
    assert_eq!(de_fwd, 250);
    assert_eq!(de_rev, 0); // Motorway default oneway forward

    // Oneway reverse (-1)
    let mut rev_tags = HashMap::new();
    rev_tags.insert("maxspeed".to_string(), "50".to_string());
    rev_tags.insert("oneway".to_string(), "-1".to_string());
    let (r_fwd, r_rev) = parse_speed_limits(&rev_tags, 6, CountryCode::EU);
    assert_eq!(r_fwd, 0);
    assert_eq!(r_rev, 50);
}

#[test]
fn test_parse_access_flags() {
    let mut tags = HashMap::new();
    tags.insert("toll".to_string(), "yes".to_string());
    tags.insert("tunnel".to_string(), "yes".to_string());
    tags.insert("bridge".to_string(), "yes".to_string());

    let flags = parse_access_flags(&tags);
    assert_ne!(flags & access_flags::TOLL, 0);
    assert_ne!(flags & access_flags::TUNNEL, 0);
    assert_ne!(flags & access_flags::BRIDGE, 0);
    assert_eq!(flags & access_flags::FERRY, 0);
}

#[test]
fn test_osm_xml_ingestion_end_to_end() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let xml_path = manifest_dir.join("tests/fixtures/sample_osm_network.xml");
    let xml_data = fs::read_to_string(&xml_path).expect("Read sample OSM XML");

    let config = OsmIngestConfig {
        bounding_box: None,
        country: CountryCode::AL,
        max_frc: 7,
        simplify_epsilon_m: 0.5,
    };

    let dataset = OsmIngestPipeline::ingest_xml(&xml_data, &config)
        .expect("Ingest OSM XML succeeded");

    // Check nodes extracted
    assert!(dataset.nodes.len() >= 6);
    // Check junction flag on central node 101
    let central_node = dataset.nodes.iter().find(|n| n.node_id == 101).expect("Found node 101");
    assert_ne!(central_node.junction_flags & junction_flags::ROUNDABOUT, 0);

    // Check edges extracted
    assert!(!dataset.edges.is_empty());
    // Find motorway edge (from Way 202)
    let motorway_edge = dataset.edges.iter().find(|e| e.frc == 0).expect("Found FRC 0 motorway edge");
    assert_eq!(motorway_edge.speed_forward, 130);
    assert_eq!(motorway_edge.speed_reverse, 0); // oneway
    assert_eq!(motorway_edge.lane_count, 4);
    assert_ne!(motorway_edge.access_flags & access_flags::TOLL, 0);

    // Check turn restrictions (relation 301: no_left_turn)
    assert_eq!(dataset.restrictions.len(), 1);
    let restriction = &dataset.restrictions[0];
    assert_eq!(restriction.restriction_type, restriction_types::NO_LEFT_TURN);
    assert_eq!(restriction.penalty_s, 0xFFFF); // Absolute prohibition

    // Validate dataset referential integrity
    assert!(dataset.validate().is_ok());
}

#[test]
fn test_osm_geojson_ingestion_end_to_end() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let geojson_path = manifest_dir.join("tests/fixtures/sample_osm_network.geojson");
    let json_data = fs::read_to_string(&geojson_path).expect("Read sample OSM GeoJSON");

    let config = OsmIngestConfig {
        bounding_box: None,
        country: CountryCode::AL,
        max_frc: 7,
        simplify_epsilon_m: 0.5,
    };

    let dataset = OsmIngestPipeline::ingest_geojson(&json_data, &config)
        .expect("Ingest GeoJSON succeeded");

    assert!(dataset.nodes.len() >= 4);
    assert!(dataset.edges.len() >= 2);
    assert!(dataset.validate().is_ok());
}

#[test]
fn test_osm_ingest_bounding_box_filtering() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let xml_path = manifest_dir.join("tests/fixtures/sample_osm_network.xml");
    let xml_data = fs::read_to_string(&xml_path).expect("Read sample OSM XML");

    // Filter to box that excludes the central network
    let disjoint_box = mmi_rebuild::geo::BoundingBox::new(50.0, 10.0, 51.0, 11.0);
    let config = OsmIngestConfig {
        bounding_box: Some(disjoint_box),
        country: CountryCode::AL,
        max_frc: 7,
        simplify_epsilon_m: 0.5,
    };

    let result = OsmIngestPipeline::ingest_xml(&xml_data, &config);
    assert!(result.is_err());
    match result.unwrap_err() {
        mmi_rebuild::osm_ingest::OsmIngestError::NoNavigableRoadways => {}
        other => panic!("Expected NoNavigableRoadways, got {:?}", other),
    }
}

#[test]
fn test_osm_ingest_malformed_xml() {
    let bad_xml = "<osm><node lat=\"invalid\"/></osm>";
    let config = OsmIngestConfig::default();
    let result = OsmIngestPipeline::ingest_xml(bad_xml, &config);
    assert!(result.is_err());
}

#[test]
fn test_osm_pbf_empty_buffer_handling() {
    let empty_pbf = [0u8; 0];
    let config = OsmIngestConfig::default();
    let result = OsmIngestPipeline::ingest_pbf(&empty_pbf, &config);
    assert!(result.is_err());
}

