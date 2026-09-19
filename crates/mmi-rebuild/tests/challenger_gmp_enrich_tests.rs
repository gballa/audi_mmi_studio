//! challenger_gmp_enrich_tests: Empirical verification and adversarial stress testing
//! for crates/mmi-rebuild/src/gmp_enrich.rs.
//!
//! Objectives:
//! 1. Cache expiration: verify records older than 30 days are pruned while records <30 days are retained.
//!    Test boundary conditions: exact 30-day boundary (2,592,000s), 1s before, 1s after, clock skew.
//! 2. Place ID retention: verify place_id is retained indefinitely in IrPoi / IrDataset,
//!    and evaluate GmpCacheManager retention behavior across pruning cycles.
//! 3. FieldMask header enforcement: verify strict compliance with Places API (New) required fields,
//!    comma-delimited syntax, token validity, and client enforcement.
//! 4. EV charging station power classification:
//!    - Ultra-Fast (>= 150 kW)
//!    - Rapid (50-149 kW)
//!    - Standard (< 50 kW)
//!    Boundary testing: 150.0 kW, 149.99 kW, 50.0 kW, 49.99 kW, 0 kW, multi-connector aggregations.
//! 5. Speed enforcement camera coordinates and commercial fuel brands:
//!    Verify coordinate precision, category assignment, bounding box spatial filtering,
//!    and investigate brand attribute population.

use tempfile::TempDir;

use mmi_rebuild::geo::{BoundingBox, IrDataset, IrPoi};
use mmi_rebuild::gmp_enrich::{
    EvConnectorType, GeocodingLocationType, GmpCacheManager, GmpClient,
    GmpConnectorAggregation, GmpEnrichmentPipeline, GmpError, GmpEvChargeOptions,
    GmpGeocodingGeometry, GmpGeocodingResult, GmpLatLng, GmpLocalizedText,
    GmpPlace, GmpPlacesSearchRequest, LiveGmpClient, OfflineFixtureGmpClient,
    PLACES_API_FIELD_MASK, TOS_CACHE_MAX_AGE_SECONDS,
};

// =============================================================================
// 1. ToS 30-Day Cache Expiration Boundary & Stress Testing
// =============================================================================

#[test]
fn test_challenger_cache_expiration_exact_boundaries() {
    let mut cache = GmpCacheManager::new();
    let now: u64 = 1_700_000_000; // Fixed deterministic baseline timestamp

    // ToS limit: exactly 30 days = 30 * 86,400 seconds = 2,592,000 seconds
    assert_eq!(TOS_CACHE_MAX_AGE_SECONDS, 2_592_000);

    // Create POIs at exact temporal boundaries:
    // T0: exactly now (0s age) -> RETAIN
    let poi_now = IrPoi::new(1, "POI Now".into(), "ev_charging".into(), 41.38, 19.65, None);
    cache.insert_poi("id_now".into(), poi_now, now);

    // T1: 29 days, 23 hours, 59 mins, 59 secs (2,591,999s age) -> RETAIN (< 30 days)
    let poi_29d = IrPoi::new(2, "POI 29d".into(), "ev_charging".into(), 41.38, 19.65, None);
    cache.insert_poi("id_29d".into(), poi_29d, now - 2_591_999);

    // T2: exactly 30 days (2,592,000s age) -> RETAIN (not > 30 days)
    let poi_30d_exact = IrPoi::new(3, "POI 30d Exact".into(), "ev_charging".into(), 41.38, 19.65, None);
    cache.insert_poi("id_30d_exact".into(), poi_30d_exact, now - 2_592_000);

    // T3: 30 days + 1 second (2,592,001s age) -> PRUNE (> 30 days)
    let poi_30d_plus_1s = IrPoi::new(4, "POI 30d + 1s".into(), "ev_charging".into(), 41.38, 19.65, None);
    cache.insert_poi("id_30d_plus_1s".into(), poi_30d_plus_1s, now - 2_592_001);

    // T4: 31 days (2,678,400s age) -> PRUNE
    let poi_31d = IrPoi::new(5, "POI 31d".into(), "fuel".into(), 48.56, 11.58, None);
    cache.insert_poi("id_31d".into(), poi_31d, now - (31 * 86_400));

    // T5: 90 days (7,776,000s age) -> PRUNE
    let poi_90d = IrPoi::new(6, "POI 90d".into(), "speed_camera".into(), 41.31, 19.82, None);
    cache.insert_poi("id_90d".into(), poi_90d, now - (90 * 86_400));

    // T6: Clock skew into the future (now + 3,600s) -> RETAIN (age 0s)
    let poi_future = IrPoi::new(7, "POI Future".into(), "fuel".into(), 41.32, 19.78, None);
    cache.insert_poi("id_future".into(), poi_future, now + 3600);

    assert_eq!(cache.pois.len(), 7, "All 7 POIs initially in cache");

    // Empirical inspection of `is_stale()` before pruning
    assert!(!cache.pois.get("id_now").unwrap().is_stale(now), "T0 must not be stale");
    assert!(!cache.pois.get("id_29d").unwrap().is_stale(now), "T1 must not be stale");
    assert!(!cache.pois.get("id_30d_exact").unwrap().is_stale(now), "T2 must not be stale");
    assert!(cache.pois.get("id_30d_plus_1s").unwrap().is_stale(now), "T3 MUST be stale");
    assert!(cache.pois.get("id_31d").unwrap().is_stale(now), "T4 MUST be stale");
    assert!(cache.pois.get("id_90d").unwrap().is_stale(now), "T5 MUST be stale");
    assert!(!cache.pois.get("id_future").unwrap().is_stale(now), "T6 future clock skew must not be stale");

    // Pre-pruning retrieval verification: get_poi() enforces TTL check on read
    assert!(cache.get_poi("id_now", now).is_some());
    assert!(cache.get_poi("id_29d", now).is_some());
    assert!(cache.get_poi("id_30d_exact", now).is_some());
    assert!(cache.get_poi("id_30d_plus_1s", now).is_none(), "Stale entry must not be retrieved");
    assert!(cache.get_poi("id_31d", now).is_none(), "Stale entry must not be retrieved");
    assert!(cache.get_poi("id_90d", now).is_none(), "Stale entry must not be retrieved");
    assert!(cache.get_poi("id_future", now).is_some());

    // Execute prune
    let pruned_count = cache.prune_stale_entries(now);
    assert_eq!(pruned_count, 3, "Exactly 3 stale entries (T3, T4, T5) must be pruned");
    assert_eq!(cache.pois.len(), 4, "Exactly 4 valid entries must remain");

    // Remaining keys
    assert!(cache.pois.contains_key("id_now"));
    assert!(cache.pois.contains_key("id_29d"));
    assert!(cache.pois.contains_key("id_30d_exact"));
    assert!(cache.pois.contains_key("id_future"));

    // Pruned keys
    assert!(!cache.pois.contains_key("id_30d_plus_1s"));
    assert!(!cache.pois.contains_key("id_31d"));
    assert!(!cache.pois.contains_key("id_90d"));
}

#[test]
fn test_challenger_geocoding_cache_expiration_boundaries() {
    let mut cache = GmpCacheManager::new();
    let now: u64 = 1_700_000_000;

    let create_geo_result = |place_id: &str, addr: &str| GmpGeocodingResult {
        place_id: place_id.to_string(),
        formatted_address: addr.to_string(),
        geometry: GmpGeocodingGeometry {
            location: GmpLatLng { latitude: 48.7842, longitude: 11.4116 },
            location_type: GeocodingLocationType::Rooftop,
        },
        types: vec!["street_address".into()],
    };

    // Insert geocode queries at boundaries
    cache.insert_geocode("query_fresh".into(), create_geo_result("geo_01", "Addr Fresh"), now - 100);
    cache.insert_geocode("query_exact_30d".into(), create_geo_result("geo_02", "Addr 30d"), now - 2_592_000);
    cache.insert_geocode("query_stale".into(), create_geo_result("geo_03", "Addr Stale"), now - 2_592_001);

    // Verify retrieval
    assert!(cache.get_geocode("query_fresh", now).is_some());
    assert!(cache.get_geocode("query_exact_30d", now).is_some());
    assert!(cache.get_geocode("query_stale", now).is_none());

    // Prune
    let pruned = cache.prune_stale_entries(now);
    assert_eq!(pruned, 1);
    assert_eq!(cache.geocodes.len(), 2);
    assert!(cache.geocodes.contains_key("query_fresh"));
    assert!(cache.geocodes.contains_key("query_exact_30d"));
    assert!(!cache.geocodes.contains_key("query_stale"));
}

#[test]
fn test_challenger_cache_persistence_stress() {
    let temp_dir = TempDir::new().unwrap();
    let cache_file = temp_dir.path().join("stress_cache.json");

    let mut cache = GmpCacheManager::new();
    let now: u64 = 1_700_000_000;

    // Populate 1,000 synthetic POIs: 350 stale, 650 fresh
    for i in 0..1000 {
        let is_stale = i < 350;
        let age = if is_stale {
            (31 * 86_400) + (i as u64 * 100) // 31+ days old
        } else {
            (10 * 86_400) + (i as u64 * 10)  // 10-15 days old
        };
        let poi = IrPoi::new(
            i as u32,
            format!("Synthetic POI {}", i),
            "ev_charging".into(),
            40.0 + (i as f64 * 0.001),
            15.0 + (i as f64 * 0.001),
            Some("Brand".into()),
        );
        cache.insert_poi(format!("place_id_{:04}", i), poi, now.saturating_sub(age));
    }

    assert_eq!(cache.pois.len(), 1000);

    // Save to disk
    cache.save_to_file(&cache_file).expect("Save cache succeeded");

    // Reload from disk
    let mut reloaded = GmpCacheManager::load_from_file(&cache_file).expect("Load cache succeeded");
    assert_eq!(reloaded.pois.len(), 1000);

    // Prune stale entries
    let pruned = reloaded.prune_stale_entries(now);
    assert_eq!(pruned, 350, "Must prune exactly 350 stale entries");
    assert_eq!(reloaded.pois.len(), 650, "Must retain exactly 650 fresh entries");

    // Re-save pruned cache
    reloaded.save_to_file(&cache_file).expect("Re-save cache succeeded");
    let reloaded_pruned = GmpCacheManager::load_from_file(&cache_file).expect("Reload pruned succeeded");
    assert_eq!(reloaded_pruned.pois.len(), 650);
}

// =============================================================================
// 2. Place ID Indefinite Retention Verification
// =============================================================================

#[test]
fn test_challenger_place_id_retention_in_ir_dataset() {
    // ToS Section 3.2.3 specifically exempts Place IDs from the 30-day deletion rule:
    // Place IDs are permitted to be retained indefinitely as permanent cross-references.
    let bbox = BoundingBox::new(41.0, 19.0, 42.0, 20.0);
    let mut dataset = IrDataset::new(bbox, Some("AL".to_string()));

    let mut pipeline = GmpEnrichmentPipeline::new_offline();
    let added = pipeline.enrich_dataset(&mut dataset).expect("Enrichment succeeded");
    assert!(added > 0);

    // Verify every enriched POI has a non-empty google_place_id
    for poi in &dataset.pois {
        let place_id = poi.google_place_id.as_ref().expect("POI must have google_place_id");
        assert!(!place_id.is_empty(), "Place ID must not be empty string");
        assert!(place_id.starts_with("ChIJ"), "Standard Google Place ID prefix is ChIJ: {}", place_id);
    }

    // Verify that IrPoi has NO expiration or TTL field — it is an immutable, permanent record
    // in the navigation database IR
    let serialized_pois = serde_json::to_string(&dataset.pois).unwrap();
    assert!(!serialized_pois.contains("cached_at"), "IrPoi must not carry cache TTL metadata");
    assert!(!serialized_pois.contains("is_stale"), "IrPoi is permanent compiled geodata");
}

#[test]
fn test_challenger_place_id_eviction_behavior_in_cache_manager() {
    // Adversarial finding: In GmpCacheManager, CachedPoiRecord bundles place_id with IrPoi.
    // When prune_stale_entries() is invoked, self.pois.retain(...) removes the entire
    // CachedPoiRecord. Thus, GmpCacheManager DOES NOT maintain a tombstone or separate
    // index of retained place_ids.
    let mut cache = GmpCacheManager::new();
    let now = 1_700_000_000;
    let stale_time = now - (35 * 86_400);

    let poi = IrPoi::new(100, "Old Station".into(), "fuel".into(), 41.3, 19.7, None);
    let target_place_id = "ChIJ_test_permanent_place_id_99";
    cache.insert_poi(target_place_id.to_string(), poi, stale_time);

    assert!(cache.pois.contains_key(target_place_id));

    // Prune cache
    cache.prune_stale_entries(now);

    // Observation: GmpCacheManager purges the entire entry
    let in_cache_after_prune = cache.pois.contains_key(target_place_id);
    assert!(!in_cache_after_prune, "GmpCacheManager purges stale CachedPoiRecord from memory");

    // The place_id is preserved exclusively downstream in IrPoi / Geographic.gdb,
    // not within the GmpCacheManager temporary fetch cache.
}

// =============================================================================
// 3. FieldMask Header Strict Enforcement Testing
// =============================================================================

#[test]
fn test_challenger_fieldmask_header_structure_and_completeness() {
    // Google Maps Platform Places API (New) requires FieldMask header:
    // Missing or invalid FieldMask results in 400 Bad Request or excessive billing SKUs.
    let mask = PLACES_API_FIELD_MASK;

    // 1. Mandatory tokens
    let mandatory_tokens = [
        "places.id",
        "places.displayName",
        "places.location",
        "places.formattedAddress",
        "places.primaryType",
        "places.evChargeOptions",
    ];

    let parsed_tokens: Vec<&str> = mask.split(',').map(|s| s.trim()).collect();
    for token in mandatory_tokens {
        assert!(
            parsed_tokens.contains(&token),
            "PLACES_API_FIELD_MASK must contain token '{}', found: {:?}",
            token, parsed_tokens
        );
    }

    // 2. Prefix validation: every token must start with 'places.'
    for token in &parsed_tokens {
        assert!(
            token.starts_with("places."),
            "Invalid token in FieldMask: '{}' must begin with 'places.' prefix",
            token
        );
        assert!(!token.contains(' '), "FieldMask tokens must not contain whitespace: '{}'", token);
    }

    // 3. No duplicate tokens
    let mut unique_set = std::collections::HashSet::new();
    for token in &parsed_tokens {
        assert!(unique_set.insert(token), "Duplicate token in FieldMask: {}", token);
    }
}

#[test]
fn test_challenger_client_api_key_and_fieldmask_enforcement() {
    // 1. Live client with empty API key must reject calls immediately
    let client_empty = LiveGmpClient::new("".to_string());
    let req = GmpPlacesSearchRequest {
        text_query: "fuel".into(),
        included_type: Some("gas_station".into()),
        max_result_count: Some(10),
    };
    let err_search = client_empty.search_places(&req);
    assert!(err_search.is_err(), "Empty API key must fail search_places");
    match err_search.unwrap_err() {
        GmpError::MissingApiKey => {}
        other => panic!("Expected MissingApiKey, got {:?}", other),
    }

    let err_geo = client_empty.geocode("Audi Ingolstadt");
    assert!(err_geo.is_err(), "Empty API key must fail geocode");
    match err_geo.unwrap_err() {
        GmpError::MissingApiKey => {}
        other => panic!("Expected MissingApiKey, got {:?}", other),
    }

    // 2. Demo key client initialization
    let client_demo = LiveGmpClient::with_demo_key("AIzaSyDEMO_KEY_VALID_FORMAT_123456789".to_string());
    let res_demo = client_demo.search_places(&req);
    assert!(res_demo.is_ok(), "Demo key client successfully executes search via fallback");
}

// =============================================================================
// 4. EV Charging Station Power Classification Testing
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvPowerClassification {
    UltraFast, // >= 150 kW
    Rapid,     // 50 .. 149 kW (inclusive of 50, strictly < 150)
    Standard,  // < 50 kW
    Unspecified,
}

/// Official power classification oracle:
/// Ultra-Fast >= 150 kW, Rapid 50-149 kW, Standard < 50 kW
pub fn classify_ev_power_rating(power_kw: Option<f32>) -> EvPowerClassification {
    match power_kw {
        Some(kw) if kw.is_nan() || kw < 0.0 => EvPowerClassification::Unspecified,
        Some(kw) if kw >= 150.0 => EvPowerClassification::UltraFast,
        Some(kw) if kw >= 50.0 => EvPowerClassification::Rapid,
        Some(kw) if kw > 0.0 => EvPowerClassification::Standard,
        Some(_) => EvPowerClassification::Standard, // 0.0 kW
        None => EvPowerClassification::Unspecified,
    }
}

#[test]
fn test_challenger_ev_charging_power_classification_oracle() {
    // 1. Ultra-Fast Tier (>= 150.0 kW)
    assert_eq!(classify_ev_power_rating(Some(350.0)), EvPowerClassification::UltraFast); // Ionity / HPC
    assert_eq!(classify_ev_power_rating(Some(300.0)), EvPowerClassification::UltraFast);
    assert_eq!(classify_ev_power_rating(Some(250.0)), EvPowerClassification::UltraFast); // Tesla V3 / V4
    assert_eq!(classify_ev_power_rating(Some(175.0)), EvPowerClassification::UltraFast);
    assert_eq!(classify_ev_power_rating(Some(150.0)), EvPowerClassification::UltraFast); // Exact threshold

    // 2. Rapid Tier (50.0 kW to 149.99 kW)
    assert_eq!(classify_ev_power_rating(Some(149.999)), EvPowerClassification::Rapid); // Threshold - epsilon
    assert_eq!(classify_ev_power_rating(Some(120.0)), EvPowerClassification::Rapid);
    assert_eq!(classify_ev_power_rating(Some(100.0)), EvPowerClassification::Rapid);
    assert_eq!(classify_ev_power_rating(Some(75.0)), EvPowerClassification::Rapid);
    assert_eq!(classify_ev_power_rating(Some(50.0)), EvPowerClassification::Rapid); // Exact lower threshold

    // 3. Standard Tier (< 50.0 kW)
    assert_eq!(classify_ev_power_rating(Some(49.999)), EvPowerClassification::Standard); // Lower threshold - epsilon
    assert_eq!(classify_ev_power_rating(Some(43.0)), EvPowerClassification::Standard); // Fast AC
    assert_eq!(classify_ev_power_rating(Some(22.0)), EvPowerClassification::Standard); // Standard European 22kW AC
    assert_eq!(classify_ev_power_rating(Some(11.0)), EvPowerClassification::Standard); // Domestic 11kW Wallbox
    assert_eq!(classify_ev_power_rating(Some(7.4)), EvPowerClassification::Standard);  // Single-phase 32A
    assert_eq!(classify_ev_power_rating(Some(3.7)), EvPowerClassification::Standard);  // Single-phase 16A
    assert_eq!(classify_ev_power_rating(Some(0.0)), EvPowerClassification::Standard);

    // 4. Edge cases
    assert_eq!(classify_ev_power_rating(None), EvPowerClassification::Unspecified);
    assert_eq!(classify_ev_power_rating(Some(-10.0)), EvPowerClassification::Unspecified);
    assert_eq!(classify_ev_power_rating(Some(f32::NAN)), EvPowerClassification::Unspecified);
}

#[test]
fn test_challenger_ev_station_multi_connector_aggregation_resolution() {
    let client = OfflineFixtureGmpClient::new();
    let req = GmpPlacesSearchRequest {
        text_query: "electric vehicle charging station".into(),
        included_type: Some("electric_vehicle_charging_station".into()),
        max_result_count: Some(10),
    };

    let places = client.search_places(&req).expect("Places search succeeded");

    // Ionity: 350 kW CCS2
    let ionity = places.iter().find(|p| p.id.contains("ionity")).expect("Ionity station found");
    let ionity_opts = ionity.ev_charge_options.as_ref().unwrap();
    let ionity_max_kw = ionity_opts.connector_aggregation.iter()
        .filter_map(|a| a.max_charge_rate_kw)
        .fold(0.0f64, f64::max) as f32;
    assert_eq!(ionity_max_kw, 350.0);
    assert_eq!(classify_ev_power_rating(Some(ionity_max_kw)), EvPowerClassification::UltraFast);

    // Tesla Supercharger: 250 kW dual aggregation (CCS2 and Tesla)
    let tesla = places.iter().find(|p| p.id.contains("tesla")).expect("Tesla station found");
    let tesla_opts = tesla.ev_charge_options.as_ref().unwrap();
    let tesla_max_kw = tesla_opts.connector_aggregation.iter()
        .filter_map(|a| a.max_charge_rate_kw)
        .fold(0.0f64, f64::max) as f32;
    assert_eq!(tesla_max_kw, 250.0);
    assert_eq!(classify_ev_power_rating(Some(tesla_max_kw)), EvPowerClassification::UltraFast);

    // Synthetic station with mixed AC and DC chargers: 22 kW Type2 + 75 kW CCS2
    let mixed_place = GmpPlace {
        id: "ChIJ_mixed_station_01".into(),
        display_name: GmpLocalizedText { text: "Mixed City Hub".into(), language_code: Some("de".into()) },
        location: GmpLatLng { latitude: 48.1, longitude: 11.5 },
        formatted_address: Some("City Hub 1".into()),
        primary_type: Some("electric_vehicle_charging_station".into()),
        ev_charge_options: Some(GmpEvChargeOptions {
            connector_count: Some(4),
            connector_aggregation: vec![
                GmpConnectorAggregation {
                    connector_type: EvConnectorType::Type2,
                    max_charge_rate_kw: Some(22.0),
                    count: Some(2),
                },
                GmpConnectorAggregation {
                    connector_type: EvConnectorType::Ccs2,
                    max_charge_rate_kw: Some(75.0),
                    count: Some(2),
                },
            ],
        }),
    };

    // Resolving maximum power across connectors
    let max_kw = mixed_place.ev_charge_options.as_ref().unwrap().connector_aggregation.iter()
        .filter_map(|a| a.max_charge_rate_kw)
        .fold(0.0f64, f64::max) as f32;
    assert_eq!(max_kw, 75.0);
    assert_eq!(classify_ev_power_rating(Some(max_kw)), EvPowerClassification::Rapid);
}

// =============================================================================
// 5. Speed Cameras and Commercial Fuel Brands Verification
// =============================================================================

#[test]
fn test_challenger_speed_camera_coordinates_and_fidelity() {
    let client = OfflineFixtureGmpClient::new();
    let req = GmpPlacesSearchRequest {
        text_query: "speed camera".into(),
        included_type: Some("speed_camera".into()),
        max_result_count: Some(5),
    };

    let cameras = client.search_places(&req).expect("Speed camera search succeeded");
    assert!(!cameras.is_empty(), "Must find speed cameras in fixture");

    let cam = &cameras[0];
    assert_eq!(cam.id, "ChIJ_camera_tirana_ring_01");
    assert_eq!(cam.display_name.text, "Speed Camera Tirana Outer Ring (80 km/h)");
    assert_eq!(cam.primary_type.as_deref(), Some("speed_camera"));

    // Verify coordinate accuracy
    assert_eq!(cam.location.latitude, 41.3150);
    assert_eq!(cam.location.longitude, 19.8220);

    // Verify coordinates in enriched IrDataset
    let bbox = BoundingBox::new(41.0, 19.0, 42.0, 20.0);
    let mut dataset = IrDataset::new(bbox, Some("AL".into()));
    let mut pipeline = GmpEnrichmentPipeline::new_offline();
    pipeline.enrich_dataset(&mut dataset).expect("Enrichment ok");

    let cam_poi = dataset.pois.iter().find(|p| p.category == "speed_camera").expect("Found camera POI");
    assert_eq!(cam_poi.lat, 41.3150);
    assert_eq!(cam_poi.lon, 19.8220);
    assert_eq!(cam_poi.power_kw, None, "Camera must have no EV power rating");
    assert!(cam_poi.connectors.is_empty(), "Camera must have no EV connectors");
    assert_eq!(cam_poi.google_place_id, Some("ChIJ_camera_tirana_ring_01".into()));
}

#[test]
fn test_challenger_commercial_fuel_brands_and_spatial_filtering() {
    let client = OfflineFixtureGmpClient::new();
    let req = GmpPlacesSearchRequest {
        text_query: "fuel".into(),
        included_type: Some("gas_station".into()),
        max_result_count: Some(10),
    };

    let stations = client.search_places(&req).expect("Fuel station search succeeded");
    assert!(stations.len() >= 2);

    // 1. Shell Station in Tirana, Albania
    let shell = stations.iter().find(|p| p.display_name.text.contains("Shell")).expect("Found Shell");
    assert_eq!(shell.id, "ChIJ_albania_shell_tirana_01");
    assert_eq!(shell.location.latitude, 41.3218);
    assert_eq!(shell.location.longitude, 19.7891);
    assert!(shell.formatted_address.as_ref().unwrap().contains("Tiranë"));

    // 2. Aral Station on A9 Holledau, Germany
    let aral = stations.iter().find(|p| p.display_name.text.contains("Aral")).expect("Found Aral");
    assert_eq!(aral.id, "ChIJ_dach_aral_a9_01");
    assert_eq!(aral.location.latitude, 48.5630);
    assert_eq!(aral.location.longitude, 11.5890);
    assert!(aral.formatted_address.as_ref().unwrap().contains("A9"));

    // 3. Spatial Bounding Box Filtering Test:
    // Albania bounding box must contain Shell and EXCLUDE Aral
    let albania_bbox = BoundingBox::new(41.0, 19.0, 42.0, 20.0);
    let mut albania_dataset = IrDataset::new(albania_bbox, Some("AL".into()));
    let mut pipeline = GmpEnrichmentPipeline::new_offline();
    pipeline.enrich_dataset(&mut albania_dataset).expect("Enrichment ok");

    let albania_fuel_pois: Vec<&IrPoi> = albania_dataset.pois.iter().filter(|p| p.category == "fuel").collect();
    assert_eq!(albania_fuel_pois.len(), 1, "Albania dataset must contain exactly 1 fuel station");
    assert!(albania_fuel_pois[0].name.contains("Shell"));
    assert!(!albania_fuel_pois[0].name.contains("Aral"));

    // DACH bounding box must contain Aral and EXCLUDE Shell
    let dach_bbox = BoundingBox::new(47.0, 10.0, 49.5, 13.0);
    let mut dach_dataset = IrDataset::new(dach_bbox, Some("DE".into()));
    pipeline.enrich_dataset(&mut dach_dataset).expect("Enrichment ok");

    let dach_fuel_pois: Vec<&IrPoi> = dach_dataset.pois.iter().filter(|p| p.category == "fuel").collect();
    assert_eq!(dach_fuel_pois.len(), 1, "DACH dataset must contain exactly 1 fuel station");
    assert!(dach_fuel_pois[0].name.contains("Aral"));
    assert!(!dach_fuel_pois[0].name.contains("Shell"));
}
