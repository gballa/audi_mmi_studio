use std::time::{SystemTime, UNIX_EPOCH};
use tempfile::TempDir;

use mmi_rebuild::geo::{BoundingBox, IrDataset};
use mmi_rebuild::gmp_enrich::{
    EvConnectorType, GeocodingLocationType, GmpCacheManager, GmpClient,
    GmpEnrichmentPipeline, GmpPlacesSearchRequest, OfflineFixtureGmpClient,
    PLACES_API_FIELD_MASK,
};

#[test]
fn test_places_api_new_request_fieldmask() {
    // Required by Google Maps Platform best practices
    assert!(PLACES_API_FIELD_MASK.contains("places.id"));
    assert!(PLACES_API_FIELD_MASK.contains("places.displayName"));
    assert!(PLACES_API_FIELD_MASK.contains("places.location"));
    assert!(PLACES_API_FIELD_MASK.contains("places.formattedAddress"));
    assert!(PLACES_API_FIELD_MASK.contains("places.primaryType"));
    assert!(PLACES_API_FIELD_MASK.contains("places.evChargeOptions"));
}

#[test]
fn test_offline_fixture_client_poi_search() {
    let client = OfflineFixtureGmpClient::new();

    // Query EV charging stations
    let ev_req = GmpPlacesSearchRequest {
        text_query: "EV charger".to_string(),
        included_type: Some("electric_vehicle_charging_station".to_string()),
        max_result_count: Some(10),
    };
    let ev_results = client.search_places(&ev_req).expect("EV search succeeded");
    assert!(!ev_results.is_empty());

    // Verify Ionity 350kW CCS2 charger in Albania
    let ionity = ev_results.iter().find(|p| p.id.contains("ionity")).expect("Found Ionity station");
    assert_eq!(ionity.display_name.text, "Ionity Vorë EV Ultra-Fast Charging");
    assert_eq!(ionity.location.latitude, 41.3892);
    assert_eq!(ionity.location.longitude, 19.6548);

    let opts = ionity.ev_charge_options.as_ref().expect("Has EV charge options");
    assert_eq!(opts.connector_count, Some(6));
    assert_eq!(opts.connector_aggregation[0].connector_type, EvConnectorType::Ccs2);
    assert_eq!(opts.connector_aggregation[0].max_charge_rate_kw, Some(350.0));

    // Verify Tesla Supercharger in Germany
    let tesla = ev_results.iter().find(|p| p.id.contains("tesla")).expect("Found Tesla station");
    assert!(tesla.display_name.text.contains("Tesla Supercharger"));
    assert_eq!(tesla.location.latitude, 48.1498);
}

#[test]
fn test_commercial_fuel_brand_detection() {
    let client = OfflineFixtureGmpClient::new();

    let fuel_req = GmpPlacesSearchRequest {
        text_query: "fuel".to_string(),
        included_type: Some("gas_station".to_string()),
        max_result_count: Some(10),
    };
    let fuel_results = client.search_places(&fuel_req).expect("Fuel search succeeded");
    assert!(fuel_results.len() >= 2);

    let shell = fuel_results.iter().find(|p| p.display_name.text.contains("Shell")).expect("Found Shell");
    assert!(shell.formatted_address.as_ref().unwrap().contains("Tiranë"));

    let aral = fuel_results.iter().find(|p| p.display_name.text.contains("Aral")).expect("Found Aral");
    assert!(aral.formatted_address.as_ref().unwrap().contains("A9"));
}

#[test]
fn test_speed_enforcement_camera_enrichment() {
    let client = OfflineFixtureGmpClient::new();

    let camera_req = GmpPlacesSearchRequest {
        text_query: "speed camera".to_string(),
        included_type: Some("speed_camera".to_string()),
        max_result_count: Some(5),
    };
    let cameras = client.search_places(&camera_req).expect("Camera search succeeded");
    assert!(!cameras.is_empty());

    let tirana_camera = &cameras[0];
    assert!(tirana_camera.display_name.text.contains("80 km/h"));
    assert_eq!(tirana_camera.location.latitude, 41.3150);
}

#[test]
fn test_rooftop_geocoding_precision() {
    let client = OfflineFixtureGmpClient::new();

    // Audi AG Headquarters Ingolstadt
    let audi_geo = client.geocode("Auto-Union-Straße 1, 85057 Ingolstadt")
        .expect("Geocode Audi HQ succeeded");
    assert_eq!(audi_geo.geometry.location_type, GeocodingLocationType::Rooftop);
    assert!((audi_geo.geometry.location.latitude - 48.7842).abs() < 1e-4);
    assert!((audi_geo.geometry.location.longitude - 11.4116).abs() < 1e-4);

    // Skanderbeg Square Tirana
    let sq_geo = client.geocode("Sheshi Skënderbej, Tiranë, Albania")
        .expect("Geocode Skanderbeg Square succeeded");
    assert_eq!(sq_geo.geometry.location_type, GeocodingLocationType::Rooftop);
}

#[test]
fn test_tos_30_day_caching_and_staleness_eviction() {
    let temp_dir = TempDir::new().unwrap();
    let cache_file = temp_dir.path().join("gmp_cache.json");

    let mut cache = GmpCacheManager::new();
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    let fresh_poi = mmi_rebuild::geo::IrPoi::new(
        1,
        "Fresh Ionity Station".to_string(),
        "ev_charging".to_string(),
        41.3892,
        19.6548,
        Some("Ionity".to_string()),
    );
    let stale_poi = mmi_rebuild::geo::IrPoi::new(
        2,
        "Stale Station".to_string(),
        "fuel".to_string(),
        41.3200,
        19.7800,
        Some("Shell".to_string()),
    );

    // Insert fresh POI (cached today)
    cache.insert_poi("ChIJ_fresh_01".to_string(), fresh_poi, now);

    // Insert stale POI (cached 31 days ago: 31 * 86,400s)
    let stale_timestamp = now - (31 * 86_400);
    cache.insert_poi("ChIJ_stale_01".to_string(), stale_poi, stale_timestamp);

    assert_eq!(cache.pois.len(), 2);

    // Retrieval checks against current time:
    // Fresh POI must be returned
    assert!(cache.get_poi("ChIJ_fresh_01", now).is_some());
    // Stale POI (> 30 days) must NOT be returned per ToS §3.2.3
    assert!(cache.get_poi("ChIJ_stale_01", now).is_none());

    // Prune stale entries
    let pruned = cache.prune_stale_entries(now);
    assert_eq!(pruned, 1);
    assert_eq!(cache.pois.len(), 1);
    assert!(cache.pois.contains_key("ChIJ_fresh_01"));
    assert!(!cache.pois.contains_key("ChIJ_stale_01"));

    // Save and load cache roundtrip
    cache.save_to_file(&cache_file).expect("Save cache to file succeeded");
    let loaded_cache = GmpCacheManager::load_from_file(&cache_file).expect("Load cache succeeded");
    assert_eq!(loaded_cache.pois.len(), 1);
    assert!(loaded_cache.pois.contains_key("ChIJ_fresh_01"));
}

#[test]
fn test_gmp_enrichment_pipeline_integration() {
    let bbox = BoundingBox::new(41.0, 19.0, 42.0, 20.0); // Albania bounding box
    let mut dataset = IrDataset::new(bbox, Some("AL".to_string()));

    let mut pipeline = GmpEnrichmentPipeline::new_offline();
    let added = pipeline.enrich_dataset(&mut dataset).expect("Enrichment succeeded");

    // Must have added Albania POIs (Ionity Vorë, Shell Tirana, Speed Camera Tirana)
    assert!(added >= 3, "Expected at least 3 POIs in Albania bbox, added {}", added);
    assert_eq!(dataset.pois.len(), added);

    for poi in &dataset.pois {
        assert!(bbox.contains_point(poi.lat, poi.lon));
        assert!(poi.x_mercator != 0 || poi.y_mercator != 0);
        assert!(poi.google_place_id.is_some());
    }

    // Verify cache has cached records
    assert!(pipeline.cache().pois.len() >= 3);
}
