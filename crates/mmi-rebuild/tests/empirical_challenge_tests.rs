//! empirical_challenge_tests: Adversarial stress testing and empirical validation
//! for crates/mmi-rebuild/src/geo.rs and osm_ingest.rs.
//!
//! Evaluates:
//! 1. Sub-centimeter physical surface accuracy across European extremes (Nordkapp, Tarifa, Reykjavik, Istanbul, Alps).
//! 2. 100,000-point Morton Z-order curve bit interleaving stress test and extreme bounds (-180..180, -90..90).
//! 3. Haversine distance decimeter calculations and bounding box intersections.
//! 4. OSM parser resilience against malformed XML, GeoJSON, PBF, and invalid tags.

use std::collections::HashMap;
use mmi_rebuild::geo::{
    access_flags, junction_flags, restriction_types, BoundingBox, FixedPoint32,
    IrDataset, IrEdge, IrNode, IrTurnRestriction,
    Wgs84Point,
};
use mmi_rebuild::osm_ingest::{
    classify_frc, lane_guidance_mask,
    parse_lane_count, parse_speed_limits, parse_turn_lanes,
    CountryCode, OsmIngestConfig, OsmIngestError, OsmIngestPipeline,
};

// =============================================================================
// Task 1.1: European Extremes Sub-Centimeter Accuracy (<1 cm)
// =============================================================================

#[test]
fn test_european_extremes_sub_centimeter_accuracy() {
    // Definitive geographic coordinates of European continental and regional extremes
    let extreme_locations = [
        ("Nordkapp (Norway, Northern extreme)", 71.169493, 25.783164),
        ("Punta de Tarifa (Spain, Southern continental extreme)", 36.000278, -5.603611),
        ("Reykjavik (Iceland, North-Western extreme)", 64.146582, -21.942635),
        ("Istanbul (Turkey, South-Eastern Bosphorus gateway)", 41.008238, 28.978359),
        ("Mont Blanc (Alps, Western European topographic extreme)", 45.832620, 6.865175),
        ("Cabo da Roca (Portugal, Western continental extreme)", 38.780444, -9.498889),
        ("Gavdos (Greece, Southernmost European island)", 34.836111, 24.084722),
        ("Cape Fligely (Franz Josef Land, Northernmost European land)", 81.848611, 59.102222),
        ("Longyearbyen (Svalbard, High Arctic European settlement)", 78.223200, 15.626700),
        ("Mount Olympus (Greece, Eastern Mediterranean summit)", 40.088400, 22.358600),
        ("Tirana (Albania, Western Balkans base)", 41.3275468, 19.8186982),
        ("Ingolstadt (Germany, Audi AG Headquarters)", 48.7842104, 11.4116248),
        ("Greenwich Royal Observatory (Prime Meridian)", 51.476853, 0.000000),
        ("Null Island (Equator / Prime Meridian intersection)", 0.000000, 0.000000),
    ];

    let mut max_surface_error_m = 0.0f64;
    let mut total_surface_error_m = 0.0f64;

    for (name, lat, lon) in extreme_locations {
        let original_point = Wgs84Point::new(lat, lon)
            .unwrap_or_else(|e| panic!("Failed to construct point for {}: {:?}", name, e));

        // Transform to 32-bit signed fixed-point integer coordinate space
        let fixed_pt = original_point.to_fixed_point_32();

        // Reconstruct back to geodetic WGS84 coordinates
        let (recovered_lat, recovered_lon) = fixed_pt.to_wgs84();
        let recovered_point = Wgs84Point::new(recovered_lat, recovered_lon)
            .unwrap_or_else(|e| panic!("Failed to reconstruct recovered point for {}: {:?}", name, e));

        // Calculate geodetic great-circle physical surface error using Haversine distance
        let surface_error_m = original_point.haversine_distance_m(&recovered_point);
        total_surface_error_m += surface_error_m;
        if surface_error_m > max_surface_error_m {
            max_surface_error_m = surface_error_m;
        }

        println!(
            "Extreme: {:<55} | Error: {:6.3} mm ({:.7} m)",
            name,
            surface_error_m * 1000.0,
            surface_error_m
        );

        // Mandatory contract: sub-centimeter accuracy (< 0.010 meters = 10.0 millimeters)
        assert!(
            surface_error_m < 0.010,
            "CRITICAL: Sub-centimeter accuracy violated at {}: surface error was {:.5} m (>= 0.010 m)",
            name,
            surface_error_m
        );
    }

    let avg_error_mm = (total_surface_error_m / (extreme_locations.len() as f64)) * 1000.0;
    let max_error_mm = max_surface_error_m * 1000.0;
    println!(
        "\n--- European Extremes Precision Summary: Avg Error = {:.3} mm, Max Error = {:.3} mm ---",
        avg_error_mm, max_error_mm
    );
    assert!(max_error_mm < 10.0, "Maximum surface error exceeds 10 mm");
}

#[test]
fn test_european_dense_grid_accuracy_monte_carlo() {
    // Stress-test 10,000 synthetic points across the entire European bounding box
    // Latitudes: 34.0° to 72.0°, Longitudes: -25.0° to 45.0°
    let mut lcg_state: u64 = 0xDEADBEEFCAFE1234;
    let mut next_f64 = |min: f64, max: f64| -> f64 {
        lcg_state = lcg_state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let ratio = (lcg_state >> 11) as f64 / (1u64 << 53) as f64;
        min + ratio * (max - min)
    };

    let sample_count = 10_000;
    let mut max_error_m = 0.0f64;

    for _ in 0..sample_count {
        let lat = next_f64(34.0, 72.0);
        let lon = next_f64(-25.0, 45.0);

        let pt = Wgs84Point::new(lat, lon).expect("Valid sample point");
        let fp = pt.to_fixed_point_32();
        let (r_lat, r_lon) = fp.to_wgs84();
        let rec = Wgs84Point::new(r_lat, r_lon).expect("Valid recovered point");

        let err = pt.haversine_distance_m(&rec);
        if err > max_error_m {
            max_error_m = err;
        }

        assert!(
            err < 0.010,
            "Dense grid error violated at ({}, {}): {:.6} m",
            lat, lon, err
        );
    }

    println!(
        "Dense European Grid (10,000 points): Max Surface Error = {:.4} mm",
        max_error_m * 1000.0
    );
    assert!(max_error_m < 0.010);
}

// =============================================================================
// Task 1.2: Morton Z-Order Curve 100,000-Point Reversibility Stress Test
// =============================================================================

#[test]
fn test_morton_z_order_100k_stress_and_extreme_bounds() {
    // 1. Explicit boundary and corner conditions
    let boundary_coords = [
        (i32::MIN, i32::MIN),
        (i32::MIN, i32::MAX),
        (i32::MAX, i32::MIN),
        (i32::MAX, i32::MAX),
        (0, 0),
        (-1, -1),
        (1, 1),
        (i32::MIN + 1, i32::MAX - 1),
        (i32::MAX - 1, i32::MIN + 1),
        (-2_147_483_647, 2_147_483_647), // MMI scaled extremes
        (2_147_483_647, -2_147_483_647),
        (0x5555_5555, 0x3333_3333),
        (0x0F0F_0F0F, 0x00FF_00FF),
        (0x0000_FFFF, 0x7FFF_FFFF),
        (-0x7FFF_FFFF, 0x7FFF_FFFF),
    ];

    for &(x, y) in &boundary_coords {
        let fp = FixedPoint32::new(x, y);
        let key = fp.morton_key();
        let recovered = FixedPoint32::from_morton_key(key);
        assert_eq!(
            recovered, fp,
            "Morton boundary failure at ({}, {}): key=0x{:016X}",
            x, y, key
        );
    }

    // 2. Power-of-two bit walks (single bit set in X or Y)
    for bit in 0..31 {
        let val_pos = 1i32 << bit;
        let val_neg = -val_pos;
        for &(x, y) in &[
            (val_pos, 0), (0, val_pos), (val_pos, val_pos),
            (val_neg, 0), (0, val_neg), (val_neg, val_neg),
        ] {
            let fp = FixedPoint32::new(x, y);
            let key = fp.morton_key();
            let recovered = FixedPoint32::from_morton_key(key);
            assert_eq!(recovered, fp, "Single bit walk failure at ({}, {})", x, y);
        }
    }

    // 3. 100,000 synthetic random points spanning full i32 domain
    let mut rng: u64 = 0x9E3779B97F4A7C15;
    let mut next_i32 = || -> i32 {
        rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
        (rng >> 32) as i32
    };

    let total_points = 100_000;
    for i in 0..total_points {
        let x = next_i32();
        let y = next_i32();

        let fp = FixedPoint32::new(x, y);
        let key = fp.morton_key();
        let recovered = FixedPoint32::from_morton_key(key);

        if recovered != fp {
            panic!(
                "Morton reversibility failure at iteration {} for ({}, {}): key=0x{:016X}, got ({}, {})",
                i, x, y, key, recovered.x_coord, recovered.y_coord
            );
        }
    }

    println!("Morton Z-order 100,000 synthetic points: 100% reversible, 0 bit inversions detected.");
}

#[test]
fn test_morton_wgs84_extrema_and_quadtree_ordering() {
    // Test exact extremes -180..180, -90..90
    let extrema = [
        (-90.0, -180.0),
        (-90.0, 180.0),
        (90.0, -180.0),
        (90.0, 180.0),
        (0.0, 0.0),
    ];

    for (lat, lon) in extrema {
        let pt = Wgs84Point::new(lat, lon).expect("Valid extreme WGS84 point");
        let fp = pt.to_fixed_point_32();
        let key = fp.morton_key();
        let recovered_fp = FixedPoint32::from_morton_key(key);
        assert_eq!(recovered_fp, fp, "Extrema reversibility mismatch at ({}, {})", lat, lon);
    }

    // Test quadtree quadrant preservation:
    // Q0: x < 0, y < 0 -> ux < 2^31, uy < 2^31 -> bit 63 = 0, bit 62 = 0
    // Q1: x >= 0, y < 0 -> ux >= 2^31, uy < 2^31 -> bit 62 = 1, bit 63 = 0
    // Q2: x < 0, y >= 0 -> ux < 2^31, uy >= 2^31 -> bit 62 = 0, bit 63 = 1
    // Q3: x >= 0, y >= 0 -> ux >= 2^31, uy >= 2^31 -> bit 62 = 1, bit 63 = 1
    let p_sw = FixedPoint32::new(-100_000, -100_000);
    let p_se = FixedPoint32::new(100_000, -100_000);
    let p_nw = FixedPoint32::new(-100_000, 100_000);
    let p_ne = FixedPoint32::new(100_000, 100_000);

    let k_sw = p_sw.morton_key();
    let k_se = p_se.morton_key();
    let k_nw = p_nw.morton_key();
    let k_ne = p_ne.morton_key();

    assert!(k_sw < k_se, "SW Morton should be < SE Morton");
    assert!(k_se < k_nw, "SE Morton should be < NW Morton");
    assert!(k_nw < k_ne, "NW Morton should be < NE Morton");
}

// =============================================================================
// Task 1.3: Haversine Decimeter Calculations & Bounding Box Intersections
// =============================================================================

#[test]
fn test_haversine_decimeter_accuracy_and_edge_cases() {
    // 1. Zero distance (identical points)
    let p1 = Wgs84Point::new(48.7842, 11.4116).unwrap();
    assert_eq!(p1.haversine_distance_m(&p1), 0.0);
    assert_eq!(p1.distance_dm(&p1), 0);

    // 2. Known benchmark distances
    // London (Trafalgar Square) to Paris (Eiffel Tower) ~ 343.5 km
    let london = Wgs84Point::new(51.5074, -0.1278).unwrap();
    let paris = Wgs84Point::new(48.8566, 2.3522).unwrap();
    let dist_m = london.haversine_distance_m(&paris);
    let dist_dm = london.distance_dm(&paris);

    // Check geodetic bounds ~343,500 m
    assert!(dist_m > 340_000.0 && dist_m < 346_000.0, "London-Paris distance out of range: {}", dist_m);
    assert_eq!((dist_m * 10.0).round() as u32, dist_dm, "Decimeter rounding mismatch");

    // 3. Antipodal points (exact half circumference ~ 20,037,508 meters)
    let null_island = Wgs84Point::new(0.0, 0.0).unwrap();
    let antipodal = Wgs84Point::new(0.0, 180.0).unwrap();
    let dist_half_circ = null_island.haversine_distance_m(&antipodal);
    let expected_half_circ = std::f64::consts::PI * 6_378_137.0;
    let diff = (dist_half_circ - expected_half_circ).abs();
    assert!(diff < 1.0, "Antipodal distance diff too high: {}", diff);
    assert_eq!(null_island.distance_dm(&antipodal), (dist_half_circ * 10.0).round() as u32);

    // 4. North Pole to South Pole
    let north_pole = Wgs84Point::new(90.0, 0.0).unwrap();
    let south_pole = Wgs84Point::new(-90.0, 0.0).unwrap();
    let pole_dist = north_pole.haversine_distance_m(&south_pole);
    assert!((pole_dist - expected_half_circ).abs() < 1.0);
    assert_eq!(north_pole.distance_dm(&south_pole), (pole_dist * 10.0).round() as u32);

    // 5. Short segment (1 meter)
    // 1 meter north of Ingolstadt
    let delta_lat = (1.0f64 / 6_378_137.0f64).to_degrees();
    let p_plus_1m = Wgs84Point::new(p1.lat + delta_lat, p1.lon).unwrap();
    let short_dist_m = p1.haversine_distance_m(&p_plus_1m);
    assert!((short_dist_m - 1.0).abs() < 0.001);
    assert_eq!(p1.distance_dm(&p_plus_1m), 10); // 1.0 m = 10 dm
}

#[test]
fn test_bounding_box_adversarial_intersections() {
    let base = BoundingBox::new(40.0, 10.0, 50.0, 20.0);

    // 1. Full containment
    let inside = BoundingBox::new(42.0, 12.0, 48.0, 18.0);
    assert!(base.intersects(&inside));
    assert!(inside.intersects(&base));

    // 2. Partial overlapping in all 4 cardinal directions
    let overlap_north = BoundingBox::new(45.0, 10.0, 55.0, 20.0);
    let overlap_south = BoundingBox::new(35.0, 10.0, 45.0, 20.0);
    let overlap_east  = BoundingBox::new(40.0, 15.0, 50.0, 25.0);
    let overlap_west  = BoundingBox::new(40.0, 5.0,  50.0, 15.0);

    assert!(base.intersects(&overlap_north));
    assert!(base.intersects(&overlap_south));
    assert!(base.intersects(&overlap_east));
    assert!(base.intersects(&overlap_west));

    // 3. Exact boundary touch (shared edge or corner)
    let touch_top = BoundingBox::new(50.0, 10.0, 60.0, 20.0);
    let touch_right = BoundingBox::new(40.0, 20.0, 50.0, 30.0);
    let touch_corner = BoundingBox::new(50.0, 20.0, 60.0, 30.0);
    assert!(base.intersects(&touch_top));
    assert!(base.intersects(&touch_right));
    assert!(base.intersects(&touch_corner));

    // 4. Disjoint boxes (strictly separated)
    let disjoint_north = BoundingBox::new(50.0001, 10.0, 60.0, 20.0);
    let disjoint_east  = BoundingBox::new(40.0, 20.0001, 50.0, 30.0);
    let disjoint_diag  = BoundingBox::new(50.0001, 20.0001, 60.0, 30.0);
    assert!(!base.intersects(&disjoint_north));
    assert!(!base.intersects(&disjoint_east));
    assert!(!base.intersects(&disjoint_diag));

    // 5. Degenerate point boxes (min == max)
    let pt_in = BoundingBox::new(45.0, 15.0, 45.0, 15.0);
    let pt_out = BoundingBox::new(55.0, 15.0, 55.0, 15.0);
    let pt_border = BoundingBox::new(50.0, 20.0, 50.0, 20.0);

    assert!(base.intersects(&pt_in));
    assert!(!base.intersects(&pt_out));
    assert!(base.intersects(&pt_border));
}

// =============================================================================
// Task 2: OSM Parser Resilience Against Malformed Data and Invalid Tags
// =============================================================================

#[test]
fn test_osm_xml_malformed_inputs_graceful_handling() {
    let config = OsmIngestConfig::default();

    // 1. Truncated XML
    let truncated_xml = "<osm version=\"0.6\"><node id=\"1\" lat=\"48.0\"";
    let res1 = OsmIngestPipeline::ingest_xml(truncated_xml, &config);
    assert!(res1.is_err());

    // 2. Missing required attribute lat/lon
    let missing_attr_xml = "<osm><node id=\"1\" lat=\"48.0\"/></osm>";
    let res2 = OsmIngestPipeline::ingest_xml(missing_attr_xml, &config);
    assert!(res2.is_err());

    // 3. Non-numeric latitude/longitude
    let non_num_xml = "<osm><node id=\"1\" lat=\"not_a_number\" lon=\"11.0\"/></osm>";
    let res3 = OsmIngestPipeline::ingest_xml(non_num_xml, &config);
    assert!(res3.is_err());

    // 4. Corrupted way referencing non-existent nodes
    let dangling_way_xml = r#"
    <osm version="0.6">
        <node id="1" lat="48.0" lon="11.0"/>
        <way id="100">
            <nd ref="1"/>
            <nd ref="99999999"/>
            <tag k="highway" v="primary"/>
        </way>
    </osm>
    "#;
    // The parser should skip edges referencing missing node 99999999 without panic,
    // and since no valid 2-node edge can be formed, it gracefully yields 0 edges or NoNavigableRoadways.
    let res4 = OsmIngestPipeline::ingest_xml(dangling_way_xml, &config);
    match res4 {
        Ok(ds) => {
            assert_eq!(ds.edges.len(), 0, "Dangling node should yield 0 edges");
        }
        Err(OsmIngestError::NoNavigableRoadways) => {}
        Err(other) => panic!("Unexpected error for dangling way: {:?}", other),
    }

    // 5. Way with single node (insufficient for edge)
    let single_nd_xml = r#"
    <osm version="0.6">
        <node id="1" lat="48.0" lon="11.0"/>
        <way id="100">
            <nd ref="1"/>
            <tag k="highway" v="primary"/>
        </way>
    </osm>
    "#;
    let res5 = OsmIngestPipeline::ingest_xml(single_nd_xml, &config);
    match res5 {
        Ok(ds) => {
            assert_eq!(ds.edges.len(), 0, "Single-node way should yield 0 edges");
        }
        Err(OsmIngestError::NoNavigableRoadways) => {}
        Err(other) => panic!("Unexpected error for single-node way: {:?}", other),
    }

    // 6. Cyclic turn restriction referencing missing way
    let cyclic_rel_xml = r#"
    <osm version="0.6">
        <node id="1" lat="48.0" lon="11.0"/>
        <node id="2" lat="48.1" lon="11.1"/>
        <way id="10">
            <nd ref="1"/>
            <nd ref="2"/>
            <tag k="highway" v="primary"/>
        </way>
        <relation id="500">
            <member type="way" ref="999" role="from"/>
            <member type="node" ref="1" role="via"/>
            <member type="way" ref="10" role="to"/>
            <tag k="type" v="restriction"/>
            <tag k="restriction" v="no_left_turn"/>
        </relation>
    </osm>
    "#;
    // Should parse valid roadway and gracefully discard incomplete turn restriction
    let res6 = OsmIngestPipeline::ingest_xml(cyclic_rel_xml, &config);
    assert!(res6.is_ok());
    let ds6 = res6.unwrap();
    assert_eq!(ds6.edges.len(), 1);
    assert_eq!(ds6.restrictions.len(), 0); // Missing from_edge discarded safely
}

#[test]
fn test_osm_geojson_malformed_inputs_graceful_handling() {
    let config = OsmIngestConfig::default();

    // 1. Malformed JSON syntax
    let bad_json = "{ \"type\": \"FeatureCollection\", \"features\": [ { invalid } ] }";
    assert!(OsmIngestPipeline::ingest_geojson(bad_json, &config).is_err());

    // 2. Missing "features" array
    let no_features = "{ \"type\": \"FeatureCollection\" }";
    assert!(OsmIngestPipeline::ingest_geojson(no_features, &config).is_err());

    // 3. Feature with null geometry
    let null_geom = r#"{
        "type": "FeatureCollection",
        "features": [
            { "type": "Feature", "geometry": null, "properties": { "highway": "primary" } }
        ]
    }"#;
    let res = OsmIngestPipeline::ingest_geojson(null_geom, &config);
    assert!(res.is_err()); // No navigable roadways

    // 4. LineString with invalid coordinate types (strings/nulls)
    let malformed_coords = r#"{
        "type": "FeatureCollection",
        "features": [
            {
                "type": "Feature",
                "geometry": {
                    "type": "LineString",
                    "coordinates": [["bad", "coord"], [11.0, 48.0]]
                },
                "properties": { "highway": "primary" }
            }
        ]
    }"#;
    let res_c = OsmIngestPipeline::ingest_geojson(malformed_coords, &config);
    let _ = res_c;

    // 5. LineString with 1 coordinate (cannot form edge)
    let single_coord = r#"{
        "type": "FeatureCollection",
        "features": [
            {
                "type": "Feature",
                "geometry": {
                    "type": "LineString",
                    "coordinates": [[11.0, 48.0]]
                },
                "properties": { "highway": "primary" }
            }
        ]
    }"#;
    assert!(OsmIngestPipeline::ingest_geojson(single_coord, &config).is_err());
}

#[test]
fn test_osm_pbf_malformed_payload_and_fuzzing_resilience() {
    let config = OsmIngestConfig::default();

    // 1. Truncated PBF buffers
    let empty_buf = [0u8; 0];
    assert!(OsmIngestPipeline::ingest_pbf(&empty_buf, &config).is_err());

    let short_buf = [0x00, 0x00, 0x01]; // 3 bytes, smaller than 4-byte header length
    assert!(OsmIngestPipeline::ingest_pbf(&short_buf, &config).is_err());

    // 2. Header length claims huge size exceeding buffer
    let huge_header_buf = [0x7F, 0xFF, 0xFF, 0xFF, 0x01, 0x02, 0x03];
    assert!(OsmIngestPipeline::ingest_pbf(&huge_header_buf, &config).is_err());

    // 3. Fuzzed bytes with fake zlib magic header (0x78 0x9C) followed by random noise
    let mut fuzzed_zlib = vec![
        0x00, 0x00, 0x00, 0x04, // header_len = 4
        0xAA, 0xBB, 0xCC, 0xDD, // fake header
        0x78, 0x9C,             // zlib default compression header
    ];
    // Append 100 bytes of non-deflated garbage
    fuzzed_zlib.extend_from_slice(&[0xDE, 0xAD, 0xBE, 0xEF, 0x12, 0x34, 0x56, 0x78].repeat(12));

    // Ingesting must handle zlib decoding failure gracefully and return an error without panicking
    let res_fuzz = OsmIngestPipeline::ingest_pbf(&fuzzed_zlib, &config);
    assert!(res_fuzz.is_err());
}

#[test]
fn test_adversarial_osm_tag_parsing_resilience() {
    // 1. Functional Road Classification (FRC)
    assert_eq!(classify_frc(""), None);
    assert_eq!(classify_frc("unknown_corridor"), None);
    assert_eq!(classify_frc("MOTORWAY"), None); // Case sensitivity check
    assert_eq!(classify_frc("raceway"), None);
    assert_eq!(classify_frc("aerialway"), None);

    // 2. Lane Count Parsing
    let mut tags = HashMap::new();
    tags.insert("lanes".to_string(), "-1".to_string());
    assert_eq!(parse_lane_count(&tags, 2), 2); // Fallback on negative

    tags.insert("lanes".to_string(), "0".to_string());
    assert_eq!(parse_lane_count(&tags, 0), 4); // Fallback on zero

    tags.insert("lanes".to_string(), "99999".to_string());
    assert_eq!(parse_lane_count(&tags, 2), 2); // Fallback on u8 overflow

    tags.insert("lanes".to_string(), "255".to_string());
    assert_eq!(parse_lane_count(&tags, 2), 16); // Clamped to 16 max lanes

    tags.insert("lanes".to_string(), "3 lanes".to_string());
    assert_eq!(parse_lane_count(&tags, 2), 2); // Non-numeric fallback

    // 3. Lane Guidance Bitmasks
    let mut turn_tags = HashMap::new();
    turn_tags.insert("turn:lanes".to_string(), "unknown_token|another_bad_token".to_string());
    assert_eq!(parse_turn_lanes(&turn_tags), lane_guidance_mask::THROUGH);

    turn_tags.insert("turn:lanes".to_string(), "|||||;;;;".to_string());
    assert_eq!(parse_turn_lanes(&turn_tags), lane_guidance_mask::THROUGH);

    turn_tags.insert("turn:lanes".to_string(), "".to_string());
    assert_eq!(parse_turn_lanes(&turn_tags), lane_guidance_mask::THROUGH);

    turn_tags.insert(
        "turn:lanes".to_string(),
        "merge_to_left|merge_to_right|sharp_left|sharp_right".to_string(),
    );
    let mask = parse_turn_lanes(&turn_tags);
    assert_ne!(mask & lane_guidance_mask::MERGE_LEFT, 0);
    assert_ne!(mask & lane_guidance_mask::MERGE_RIGHT, 0);
    assert_ne!(mask & lane_guidance_mask::SHARP_LEFT, 0);
    assert_ne!(mask & lane_guidance_mask::SHARP_RIGHT, 0);

    // 4. Speed Limits and Fallbacks
    let mut speed_tags = HashMap::new();
    speed_tags.insert("maxspeed".to_string(), "warp_9".to_string());
    let (s_fwd, s_rev) = parse_speed_limits(&speed_tags, 2, CountryCode::DE);
    assert_eq!(s_fwd, 100); // Fell back to DE rural state road speed
    assert_eq!(s_rev, 100);

    speed_tags.insert("maxspeed".to_string(), "-50".to_string());
    let (neg_fwd, _) = parse_speed_limits(&speed_tags, 6, CountryCode::FR);
    assert_eq!(neg_fwd, 50); // Fell back to FR urban speed

    speed_tags.insert("maxspeed".to_string(), "walk".to_string());
    let (walk_fwd, _) = parse_speed_limits(&speed_tags, 6, CountryCode::EU);
    assert_eq!(walk_fwd, 10);

    speed_tags.insert("maxspeed".to_string(), "none".to_string());
    let (none_fwd, _) = parse_speed_limits(&speed_tags, 0, CountryCode::DE);
    assert_eq!(none_fwd, 250); // Autobahn unrestricted speed

    // Oneway variations
    speed_tags.insert("oneway".to_string(), "yes".to_string());
    let (oneway_fwd, oneway_rev) = parse_speed_limits(&speed_tags, 2, CountryCode::DE);
    assert_eq!(oneway_fwd, 250);
    assert_eq!(oneway_rev, 0);

    speed_tags.insert("oneway".to_string(), "-1".to_string());
    let (rev_fwd, rev_rev) = parse_speed_limits(&speed_tags, 2, CountryCode::DE);
    assert_eq!(rev_fwd, 0);
    assert_eq!(rev_rev, 250);
}

#[test]
fn test_ir_dataset_referential_integrity_violation_detection() {
    let bbox = BoundingBox::new(40.0, 10.0, 50.0, 20.0);
    let mut dataset = IrDataset::new(bbox, Some("TEST".to_string()));

    // Valid node
    let node0 = IrNode::from_wgs84(1, 45.0, 15.0, 100, junction_flags::NONE);
    dataset.nodes.push(node0);

    // Invalid edge referencing non-existent to_node index 1
    let edge_bad = IrEdge {
        edge_id: 1,
        from_node: 0,
        to_node: 1, // Only node 0 exists
        length_dm: 100,
        frc: 2,
        speed_forward: 80,
        speed_reverse: 80,
        lane_count: 2,
        turn_lane_mask: 0x0001,
        geometry: vec![(0, 0), (1, 1)],
        access_flags: access_flags::MOTOR_VEHICLE,
    };
    dataset.edges.push(edge_bad);

    // Validation must fail
    assert!(dataset.validate().is_err());

    // Fix node count
    let node1 = IrNode::from_wgs84(2, 45.1, 15.1, 110, junction_flags::NONE);
    dataset.nodes.push(node1);
    assert!(dataset.validate().is_ok());

    // Add invalid turn restriction referencing non-existent to_edge
    let rest_bad = IrTurnRestriction {
        from_edge: 0,
        via_node: 1,
        to_edge: 99, // edge 99 does not exist
        restriction_type: restriction_types::NO_RIGHT_TURN,
        penalty_s: 0xFFFF,
    };
    dataset.restrictions.push(rest_bad);
    assert!(dataset.validate().is_err());
}
