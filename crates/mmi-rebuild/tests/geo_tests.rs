use mmi_rebuild::geo::{
    access_flags, junction_flags, restriction_types, BoundingBox, FixedPoint32,
    IrDataset, IrEdge, IrNode, IrPoi, IrTurnRestriction, RegionalProfile,
    Wgs84Point,
};

#[test]
fn test_wgs84_web_mercator_roundtrip() {
    let test_points = [
        (41.3275, 19.8187), // Tirana, Albania
        (48.1351, 11.5820), // Munich, Germany
        (48.7842, 11.4116), // Ingolstadt (Audi HQ)
        (52.5200, 13.4050), // Berlin, Germany
        (45.4642, 9.1900),   // Milan, Italy
        (46.9480, 7.4474),   // Bern, Switzerland
        (0.0, 0.0),          // Null Island
    ];

    for (lat, lon) in test_points {
        let pt = Wgs84Point::new(lat, lon).expect("Valid point");
        let merc = pt.to_web_mercator();
        let recovered = merc.to_wgs84();

        let lat_diff = (recovered.lat - lat).abs();
        let lon_diff = (recovered.lon - lon).abs();

        assert!(
            lat_diff < 1e-6,
            "Latitude roundtrip error too large: {} vs {}",
            recovered.lat,
            lat
        );
        assert!(
            lon_diff < 1e-6,
            "Longitude roundtrip error too large: {} vs {}",
            recovered.lon,
            lon
        );
    }
}

#[test]
fn test_sub_centimeter_fixed_point_resolution() {
    // Mathematical requirement:
    // Longitude resolution: (360.0 / 2^32) * (2*pi*R) ~ 9.33 mm at equator, ~6.60 mm at 45N.
    // Latitude resolution: (180.0 / 2^32) * (pi*R) ~ 4.66 mm meridional.
    // Total error must be strictly < 1 cm (0.01 m) everywhere.
    let cities = [
        ("Tirana", 41.3275468, 19.8186982),
        ("Munich", 48.1351253, 11.5819806),
        ("Ingolstadt", 48.7842104, 11.4116248),
        ("Vienna", 48.2081743, 16.3738189),
        ("Zurich", 47.3768866, 8.5416940),
        ("Rome", 41.9027835, 12.4963655),
        ("Paris", 48.8566140, 2.3522219),
    ];

    for (name, lat, lon) in cities {
        let orig = Wgs84Point::new(lat, lon).unwrap();
        let fp = orig.to_fixed_point_32();
        let (rec_lat, rec_lon) = fp.to_wgs84();
        let recovered = Wgs84Point::new(rec_lat, rec_lon).unwrap();

        let dist_error_m = orig.haversine_distance_m(&recovered);
        println!(
            "City: {:<12} | Surface error: {:.4} mm ({:.6} m)",
            name,
            dist_error_m * 1000.0,
            dist_error_m
        );

        assert!(
            dist_error_m < 0.010, // Strictly less than 1 centimeter (10 mm)
            "Precision violation for {}: surface error was {:.4} m, exceeding 1 cm limit",
            name,
            dist_error_m
        );
    }
}

#[test]
fn test_morton_z_order_interleave_deinterleave() {
    let coordinates = [
        (0i32, 0i32),
        (100, 200),
        (-500, 300),
        (-1_000_000, -2_000_000),
        (i32::MAX - 10, i32::MAX - 20),
        (i32::MIN + 10, i32::MIN + 20),
    ];

    for (x, y) in coordinates {
        let fp = FixedPoint32::new(x, y);
        let morton = fp.morton_key();
        let recovered = FixedPoint32::from_morton_key(morton);

        assert_eq!(
            recovered.x_coord, x,
            "Morton X mismatch for ({}, {}) -> key {}",
            x, y, morton
        );
        assert_eq!(
            recovered.y_coord, y,
            "Morton Y mismatch for ({}, {}) -> key {}",
            x, y, morton
        );
    }
}

#[test]
fn test_morton_spatial_locality() {
    // Points close in 2D space should have closer Morton codes than distant points
    let p_base = FixedPoint32::new(1000, 1000);
    let p_near = FixedPoint32::new(1005, 1003);
    let p_far = FixedPoint32::new(500_000, 500_000);

    let k_base = p_base.morton_key();
    let k_near = p_near.morton_key();
    let k_far = p_far.morton_key();

    let diff_near = if k_near > k_base { k_near - k_base } else { k_base - k_near };
    let diff_far = if k_far > k_base { k_far - k_base } else { k_base - k_far };

    assert!(
        diff_near < diff_far,
        "Morton spatial locality failure: near diff {} vs far diff {}",
        diff_near,
        diff_far
    );
}

#[test]
fn test_haversine_distance_and_decimeter() {
    // Munich (Marienplatz) to Ingolstadt (Audi Forum) ~ 70 km
    let pt_munich = Wgs84Point::new(48.1374, 11.5755).unwrap();
    let pt_ingolstadt = Wgs84Point::new(48.7842, 11.4116).unwrap();

    let dist_m = pt_munich.haversine_distance_m(&pt_ingolstadt);
    let dist_dm = pt_munich.distance_dm(&pt_ingolstadt);

    assert!(dist_m > 69_000.0 && dist_m < 75_000.0);
    assert_eq!((dist_m * 10.0).round() as u32, dist_dm);
}

#[test]
fn test_ir_dataset_serialization_and_validation() {
    let bbox = BoundingBox::new(41.0, 19.0, 42.0, 20.0);
    let mut dataset = IrDataset::new(bbox, Some("AL".to_string()));

    let node0 = IrNode::from_wgs84(1, 41.3275, 19.8187, 100, junction_flags::NONE);
    let node1 = IrNode::from_wgs84(2, 41.3300, 19.8187, 105, junction_flags::ROUNDABOUT);
    dataset.nodes.push(node0);
    dataset.nodes.push(node1);

    let edge0 = IrEdge {
        edge_id: 1,
        from_node: 0,
        to_node: 1,
        length_dm: 2780, // ~278 m
        frc: 0,
        speed_forward: 130,
        speed_reverse: 0,
        lane_count: 4,
        turn_lane_mask: 0x0001,
        geometry: vec![(100, 200), (100, 250)],
        access_flags: access_flags::TOLL | access_flags::MOTOR_VEHICLE,
    };
    dataset.edges.push(edge0);

    let rest0 = IrTurnRestriction {
        from_edge: 0,
        via_node: 1,
        to_edge: 0,
        restriction_type: restriction_types::NO_U_TURN,
        penalty_s: 0xFFFF,
    };
    dataset.restrictions.push(rest0);

    let poi = IrPoi::new(
        1,
        "Ionity Charger".to_string(),
        "ev_charging".to_string(),
        41.3280,
        19.8190,
        Some("Ionity".to_string()),
    );
    dataset.pois.push(poi);

    // Validation should succeed
    assert!(dataset.validate().is_ok());

    // JSON roundtrip
    let json = dataset.to_json().expect("Serialization succeeded");
    let recovered = IrDataset::from_json(&json).expect("Deserialization succeeded");

    assert_eq!(recovered.nodes.len(), 2);
    assert_eq!(recovered.edges.len(), 1);
    assert_eq!(recovered.restrictions.len(), 1);
    assert_eq!(recovered.pois.len(), 1);
    assert_eq!(recovered.region_profile, Some("AL".to_string()));
}

#[test]
fn test_regional_profiles() {
    let p_al = RegionalProfile::from_code("AL").expect("Found AL");
    assert_eq!(p_al.code, "AL");
    assert_eq!(p_al.max_volumes, 1);
    assert!(p_al.bbox.contains_point(41.3275, 19.8187)); // Tirana is inside AL

    let p_dach = RegionalProfile::from_code("DACH").expect("Found DACH");
    assert_eq!(p_dach.code, "DACH");
    assert_eq!(p_dach.max_volumes, 3);
    assert!(p_dach.bbox.contains_point(48.1351, 11.5820)); // Munich is inside DACH

    let p_ece = RegionalProfile::from_code("ECE").expect("Found ECE");
    assert_eq!(p_ece.code, "ECE");
    assert_eq!(p_ece.max_volumes, 23);
    assert_eq!(p_ece.target_size_bytes, 28_185_247_890); // ~28.19 GB genuine footprint
    assert!(p_ece.bbox.contains_point(41.3275, 19.8187)); // Tirana inside ECE
    assert!(p_ece.bbox.contains_point(48.1351, 11.5820)); // Munich inside ECE
    assert!(p_ece.bbox.contains_point(52.5200, 13.4050)); // Berlin inside ECE
}
