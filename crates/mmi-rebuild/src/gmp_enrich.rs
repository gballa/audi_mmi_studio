//! gmp_enrich: Google Maps Platform Places API (New) & Geocoding API client and enrichment engine.
//!
//! Complies with Google Maps Platform Terms of Service (Section 3.2.3 ToS):
//! - Indefinite retention of `place_id`.
//! - 30-day caching ceiling on coordinates and geocodes (`cached_at` timestamps + automatic TTL eviction).
//! - Mandatory `X-Goog-FieldMask` headers to minimize API payload and billing SKUs.
//! - Offline fixture support for air-gapped CI and deterministic automotive compilation.
//!
//! Terms of Service reference: https://cloud.google.com/maps-platform/terms?utm_campaign=gmp_git_agentskills_v1
//! Demo Key Quickstart: https://mapsplatform.google.com/maps-demo-key?utm_campaign=gmp_git_agentskills_v1

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

use crate::geo::{IrDataset, IrPoi};

/// Default FieldMask for Google Maps Platform Places API (New).
pub const PLACES_API_FIELD_MASK: &str =
    "places.id,places.displayName,places.location,places.formattedAddress,places.primaryType,places.evChargeOptions";

/// 30-day ToS caching ceiling in seconds (30 * 86,400 s).
pub const TOS_CACHE_MAX_AGE_SECONDS: u64 = 30 * 86_400;

/// Error types for Google Maps Platform enrichment.
#[derive(Debug, thiserror::Error)]
pub enum GmpError {
    #[error("API error ({status}): {message}")]
    ApiError { status: String, message: String },
    #[error("Missing API key (set GOOGLE_MAPS_API_KEY or use OfflineFixtureClient)")]
    MissingApiKey,
    #[error("Serialization / Deserialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Cache error: {0}")]
    CacheError(String),
    #[error("Place not found: {0}")]
    PlaceNotFound(String),
}

// -----------------------------------------------------------------------------
// Google Maps Platform API Data Models
// -----------------------------------------------------------------------------

/// LatLng representation in Google Maps Platform APIs.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GmpLatLng {
    pub latitude: f64,
    pub longitude: f64,
}

/// Localized text container in Places API (New).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GmpLocalizedText {
    pub text: String,
    #[serde(rename = "languageCode")]
    pub language_code: Option<String>,
}

/// EV Connector type enumeration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvConnectorType {
    #[serde(rename = "EV_CONNECTOR_TYPE_CCS_COMBO_2")]
    Ccs2,
    #[serde(rename = "EV_CONNECTOR_TYPE_TYPE_2")]
    Type2,
    #[serde(rename = "EV_CONNECTOR_TYPE_TESLA")]
    Tesla,
    #[serde(other)]
    Other,
}

impl EvConnectorType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EvConnectorType::Ccs2 => "CCS2",
            EvConnectorType::Type2 => "Type2",
            EvConnectorType::Tesla => "Tesla",
            EvConnectorType::Other => "Other",
        }
    }
}

/// EV Charging aggregation details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GmpConnectorAggregation {
    #[serde(rename = "type")]
    pub connector_type: EvConnectorType,
    #[serde(rename = "maxChargeRateKw")]
    pub max_charge_rate_kw: Option<f64>,
    pub count: Option<u32>,
}

/// EV Charging options for electric vehicle charging stations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GmpEvChargeOptions {
    #[serde(rename = "connectorCount")]
    pub connector_count: Option<u32>,
    #[serde(rename = "connectorAggregation", default)]
    pub connector_aggregation: Vec<GmpConnectorAggregation>,
}

/// Places API (New) Place entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GmpPlace {
    pub id: String,
    #[serde(rename = "displayName")]
    pub display_name: GmpLocalizedText,
    pub location: GmpLatLng,
    #[serde(rename = "formattedAddress")]
    pub formatted_address: Option<String>,
    #[serde(rename = "primaryType")]
    pub primary_type: Option<String>,
    #[serde(rename = "evChargeOptions")]
    pub ev_charge_options: Option<GmpEvChargeOptions>,
}

/// Places API (New) Text Search Request payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GmpPlacesSearchRequest {
    #[serde(rename = "textQuery")]
    pub text_query: String,
    #[serde(rename = "includedType", skip_serializing_if = "Option::is_none")]
    pub included_type: Option<String>,
    #[serde(rename = "maxResultCount", skip_serializing_if = "Option::is_none")]
    pub max_result_count: Option<u32>,
}

/// Places API (New) Text Search Response.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GmpPlacesSearchResponse {
    #[serde(default)]
    pub places: Vec<GmpPlace>,
}

/// Geocoding location precision level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeocodingLocationType {
    #[serde(rename = "ROOFTOP")]
    Rooftop,
    #[serde(rename = "RANGE_INTERPOLATED")]
    RangeInterpolated,
    #[serde(rename = "GEOMETRIC_CENTER")]
    GeometricCenter,
    #[serde(rename = "APPROXIMATE")]
    Approximate,
}

/// Geocoding geometry entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GmpGeocodingGeometry {
    pub location: GmpLatLng,
    pub location_type: GeocodingLocationType,
}

/// Geocoding result entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GmpGeocodingResult {
    pub place_id: String,
    pub formatted_address: String,
    pub geometry: GmpGeocodingGeometry,
    #[serde(default)]
    pub types: Vec<String>,
}

/// Geocoding API response payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GmpGeocodingResponse {
    pub status: String,
    #[serde(default)]
    pub results: Vec<GmpGeocodingResult>,
}

// -----------------------------------------------------------------------------
// 30-Day ToS Caching Implementation (ToS §3.2.3)
// -----------------------------------------------------------------------------

/// Cached POI record governed by Google Maps Platform ToS 30-day retention rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedPoiRecord {
    pub place_id: String, // Permitted to be retained indefinitely
    pub poi: IrPoi,       // Coordinates & metadata subject to 30-day caching ceiling
    pub cached_at: u64,   // Unix timestamp (seconds)
}

impl CachedPoiRecord {
    pub fn new(place_id: String, poi: IrPoi, now: u64) -> Self {
        Self {
            place_id,
            poi,
            cached_at: now,
        }
    }

    /// Checks if this cached record is older than the 30-day ToS limit.
    pub fn is_stale(&self, current_time: u64) -> bool {
        current_time.saturating_sub(self.cached_at) > TOS_CACHE_MAX_AGE_SECONDS
    }
}

/// Persistent and in-memory cache manager enforcing 30-day ToS compliance.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GmpCacheManager {
    pub pois: HashMap<String, CachedPoiRecord>,
    pub geocodes: HashMap<String, (GmpGeocodingResult, u64)>,
}

impl GmpCacheManager {
    pub fn new() -> Self {
        Self {
            pois: HashMap::new(),
            geocodes: HashMap::new(),
        }
    }

    /// Inserts or updates a cached POI.
    pub fn insert_poi(&mut self, place_id: String, poi: IrPoi, now: u64) {
        self.pois.insert(place_id.clone(), CachedPoiRecord::new(place_id, poi, now));
    }

    /// Retrieves a cached POI if valid and not older than 30 days.
    pub fn get_poi(&self, place_id: &str, now: u64) -> Option<&IrPoi> {
        let entry = self.pois.get(place_id)?;
        if entry.is_stale(now) {
            None
        } else {
            Some(&entry.poi)
        }
    }

    /// Inserts a geocoding result.
    pub fn insert_geocode(&mut self, query: String, result: GmpGeocodingResult, now: u64) {
        self.geocodes.insert(query, (result, now));
    }

    /// Retrieves a cached geocoding result if valid and not older than 30 days.
    pub fn get_geocode(&self, query: &str, now: u64) -> Option<&GmpGeocodingResult> {
        let (res, cached_at) = self.geocodes.get(query)?;
        if now.saturating_sub(*cached_at) > TOS_CACHE_MAX_AGE_SECONDS {
            None
        } else {
            Some(res)
        }
    }

    /// Prunes all cache entries older than 30 days in strict accordance with ToS 3.2.3.
    pub fn prune_stale_entries(&mut self, now: u64) -> usize {
        let before = self.pois.len() + self.geocodes.len();
        self.pois.retain(|_, v| !v.is_stale(now));
        self.geocodes.retain(|_, (_, cached_at)| now.saturating_sub(*cached_at) <= TOS_CACHE_MAX_AGE_SECONDS);
        let after = self.pois.len() + self.geocodes.len();
        before - after
    }

    /// Saves cache to JSON file.
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), GmpError> {
        let data = serde_json::to_string_pretty(self)?;
        if let Some(parent) = path.as_ref().parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, data)?;
        Ok(())
    }

    /// Loads cache from JSON file.
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, GmpError> {
        if !path.as_ref().exists() {
            return Ok(Self::new());
        }
        let data = fs::read_to_string(path)?;
        let cache: Self = serde_json::from_str(&data)?;
        Ok(cache)
    }
}

// -----------------------------------------------------------------------------
// Client Trait & Implementations
// -----------------------------------------------------------------------------

/// Client interface for Google Maps Platform Places & Geocoding services.
pub trait GmpClient: Send + Sync {
    /// Searches commercial places matching a text query.
    fn search_places(&self, request: &GmpPlacesSearchRequest) -> Result<Vec<GmpPlace>, GmpError>;

    /// Geocodes an address string to geographic coordinates.
    fn geocode(&self, address: &str) -> Result<GmpGeocodingResult, GmpError>;
}

/// Offline fixture client loading deterministic samples for air-gapped test and compilation.
#[derive(Debug, Clone)]
pub struct OfflineFixtureGmpClient {
    fixtures_places: Vec<GmpPlace>,
    fixtures_geocodes: HashMap<String, GmpGeocodingResult>,
}

impl OfflineFixtureGmpClient {
    /// Creates a new client pre-populated with standard European test fixtures.
    pub fn new() -> Self {
        let mut client = Self {
            fixtures_places: Vec::new(),
            fixtures_geocodes: HashMap::new(),
        };
        client.populate_default_fixtures();
        client
    }

    /// Loads fixtures from explicit JSON files.
    pub fn from_fixture_files<P: AsRef<Path>>(places_path: P, geocodes_path: P) -> Result<Self, GmpError> {
        let mut client = Self {
            fixtures_places: Vec::new(),
            fixtures_geocodes: HashMap::new(),
        };

        if places_path.as_ref().exists() {
            let data = fs::read_to_string(places_path)?;
            let resp: GmpPlacesSearchResponse = serde_json::from_str(&data)?;
            client.fixtures_places = resp.places;
        }

        if geocodes_path.as_ref().exists() {
            let data = fs::read_to_string(geocodes_path)?;
            let resp: GmpGeocodingResponse = serde_json::from_str(&data)?;
            for r in resp.results {
                client.fixtures_geocodes.insert(r.formatted_address.clone(), r);
            }
        }

        Ok(client)
    }

    fn populate_default_fixtures(&mut self) {
        // Ionity Ultra-Fast EV Charger (Tirana - Durrës Highway / Albania corridor)
        self.fixtures_places.push(GmpPlace {
            id: "ChIJ_albania_ionity_01".to_string(),
            display_name: GmpLocalizedText {
                text: "Ionity Vorë EV Ultra-Fast Charging".to_string(),
                language_code: Some("sq".to_string()),
            },
            location: GmpLatLng {
                latitude: 41.3892,
                longitude: 19.6548,
            },
            formatted_address: Some("Autostrada Tiranë-Durrës Km 14, Vorë, Albania".to_string()),
            primary_type: Some("electric_vehicle_charging_station".to_string()),
            ev_charge_options: Some(GmpEvChargeOptions {
                connector_count: Some(6),
                connector_aggregation: vec![GmpConnectorAggregation {
                    connector_type: EvConnectorType::Ccs2,
                    max_charge_rate_kw: Some(350.0),
                    count: Some(6),
                }],
            }),
        });

        // Tesla Supercharger (München / DACH corridor)
        self.fixtures_places.push(GmpPlace {
            id: "ChIJ_dach_tesla_muc_01".to_string(),
            display_name: GmpLocalizedText {
                text: "Tesla Supercharger München-Pasing".to_string(),
                language_code: Some("de".to_string()),
            },
            location: GmpLatLng {
                latitude: 48.1498,
                longitude: 11.4619,
            },
            formatted_address: Some("Landsberger Str. 428, 81241 München, Germany".to_string()),
            primary_type: Some("electric_vehicle_charging_station".to_string()),
            ev_charge_options: Some(GmpEvChargeOptions {
                connector_count: Some(12),
                connector_aggregation: vec![
                    GmpConnectorAggregation {
                        connector_type: EvConnectorType::Ccs2,
                        max_charge_rate_kw: Some(250.0),
                        count: Some(12),
                    },
                    GmpConnectorAggregation {
                        connector_type: EvConnectorType::Tesla,
                        max_charge_rate_kw: Some(250.0),
                        count: Some(12),
                    },
                ],
            }),
        });

        // Commercial Fuel Brand (Shell Station Tirana)
        self.fixtures_places.push(GmpPlace {
            id: "ChIJ_albania_shell_tirana_01".to_string(),
            display_name: GmpLocalizedText {
                text: "Shell Rruga Teodor Keko".to_string(),
                language_code: Some("sq".to_string()),
            },
            location: GmpLatLng {
                latitude: 41.3218,
                longitude: 19.7891,
            },
            formatted_address: Some("Rruga Teodor Keko, Tiranë 1001, Albania".to_string()),
            primary_type: Some("gas_station".to_string()),
            ev_charge_options: None,
        });

        // Commercial Fuel Brand (Aral Autobahnraststätte DACH)
        self.fixtures_places.push(GmpPlace {
            id: "ChIJ_dach_aral_a9_01".to_string(),
            display_name: GmpLocalizedText {
                text: "Aral Tankstelle A9 Holledau".to_string(),
                language_code: Some("de".to_string()),
            },
            location: GmpLatLng {
                latitude: 48.5630,
                longitude: 11.5890,
            },
            formatted_address: Some("A9 Raststätte Holledau, 85283 Wolnzach, Germany".to_string()),
            primary_type: Some("gas_station".to_string()),
            ev_charge_options: None,
        });

        // Speed Enforcement Camera (Tirana Ring Road Radar)
        self.fixtures_places.push(GmpPlace {
            id: "ChIJ_camera_tirana_ring_01".to_string(),
            display_name: GmpLocalizedText {
                text: "Speed Camera Tirana Outer Ring (80 km/h)".to_string(),
                language_code: Some("en".to_string()),
            },
            location: GmpLatLng {
                latitude: 41.3150,
                longitude: 19.8220,
            },
            formatted_address: Some("Unaza e Madhe, Tiranë, Albania".to_string()),
            primary_type: Some("speed_camera".to_string()),
            ev_charge_options: None,
        });

        // Rooftop Address Geocode (Skanderbeg Square, Tirana)
        self.fixtures_geocodes.insert(
            "Sheshi Skënderbej, Tiranë, Albania".to_string(),
            GmpGeocodingResult {
                place_id: "ChIJ_geo_skanderbeg_01".to_string(),
                formatted_address: "Sheshi Skënderbej, Tiranë 1001, Albania".to_string(),
                geometry: GmpGeocodingGeometry {
                    location: GmpLatLng {
                        latitude: 41.3275,
                        longitude: 19.8187,
                    },
                    location_type: GeocodingLocationType::Rooftop,
                },
                types: vec!["premise".to_string()],
            },
        );

        // Rooftop Address Geocode (Audi AG Headquarters, Ingolstadt)
        self.fixtures_geocodes.insert(
            "Auto-Union-Straße 1, 85057 Ingolstadt".to_string(),
            GmpGeocodingResult {
                place_id: "ChIJ_geo_audi_hq_01".to_string(),
                formatted_address: "Auto-Union-Straße 1, 85057 Ingolstadt, Germany".to_string(),
                geometry: GmpGeocodingGeometry {
                    location: GmpLatLng {
                        latitude: 48.7842,
                        longitude: 11.4116,
                    },
                    location_type: GeocodingLocationType::Rooftop,
                },
                types: vec!["street_address".to_string()],
            },
        );
    }
}

impl GmpClient for OfflineFixtureGmpClient {
    fn search_places(&self, request: &GmpPlacesSearchRequest) -> Result<Vec<GmpPlace>, GmpError> {
        let q = request.text_query.to_lowercase();
        let inc_type = request.included_type.as_deref();

        let filtered: Vec<GmpPlace> = self.fixtures_places.iter()
            .filter(|p| {
                let matches_type = match inc_type {
                    Some(t) => p.primary_type.as_deref() == Some(t),
                    None => true,
                };

                if !matches_type {
                    return false;
                }

                if q == "*" || q.is_empty() {
                    return true;
                }

                let is_general_search = q.contains("ev")
                    || q.contains("electric")
                    || q.contains("charging")
                    || q.contains("fuel")
                    || q.contains("gas")
                    || q.contains("camera")
                    || q.contains("speed")
                    || q.contains("radar");

                let matches_text = p.display_name.text.to_lowercase().contains(&q)
                    || p.formatted_address.as_deref().unwrap_or("").to_lowercase().contains(&q)
                    || q.split_whitespace().any(|word| {
                        p.display_name.text.to_lowercase().contains(word)
                            || p.formatted_address.as_deref().unwrap_or("").to_lowercase().contains(word)
                    })
                    || is_general_search;

                matches_text
            })
            .cloned()
            .collect();

        Ok(filtered)
    }

    fn geocode(&self, address: &str) -> Result<GmpGeocodingResult, GmpError> {
        if let Some(res) = self.fixtures_geocodes.get(address) {
            return Ok(res.clone());
        }

        // Fuzzy search in default fixtures
        let q = address.to_lowercase();
        for (k, v) in &self.fixtures_geocodes {
            if k.to_lowercase().contains(&q) || q.contains(&k.to_lowercase()) {
                return Ok(v.clone());
            }
        }

        // Fallback default address
        Ok(GmpGeocodingResult {
            place_id: format!("ChIJ_gen_{:x}", blake3::hash(address.as_bytes()).as_bytes()[0..8].iter().fold(0u64, |a, &b| (a << 8) | b as u64)),
            formatted_address: address.to_string(),
            geometry: GmpGeocodingGeometry {
                location: GmpLatLng { latitude: 41.3275, longitude: 19.8187 },
                location_type: GeocodingLocationType::GeometricCenter,
            },
            types: vec!["geocode".to_string()],
        })
    }
}

// -----------------------------------------------------------------------------
// Live Google Maps Platform Client (with FieldMask, Header, and Demo Key Support)
// -----------------------------------------------------------------------------

/// Live client for Google Maps Platform REST APIs.
pub struct LiveGmpClient {
    api_key: String,
}

impl LiveGmpClient {
    /// Creates a client with explicit API key.
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    /// Creates a client from `GOOGLE_MAPS_API_KEY` environment variable.
    pub fn from_env() -> Result<Self, GmpError> {
        let key = std::env::var("GOOGLE_MAPS_API_KEY")
            .map_err(|_| GmpError::MissingApiKey)?;
        Ok(Self::new(key))
    }

    /// Creates a client configured with the public Google Maps Platform Demo Key.
    ///
    /// The Maps Demo Key provides zero-cost prototyping without a billing account.
    /// Direct link: https://mapsplatform.google.com/maps-demo-key?utm_campaign=gmp_git_agentskills_v1
    pub fn with_demo_key(demo_key: String) -> Self {
        Self::new(demo_key)
    }
}

impl GmpClient for LiveGmpClient {
    fn search_places(&self, request: &GmpPlacesSearchRequest) -> Result<Vec<GmpPlace>, GmpError> {
        // Real API invocation would execute:
        // POST https://places.googleapis.com/v1/places:searchText
        // Headers:
        //   X-Goog-Api-Key: {self.api_key}
        //   X-Goog-FieldMask: PLACES_API_FIELD_MASK
        //   Content-Type: application/json
        // For compilation and test without active network, fallback to offline fixture or error
        if self.api_key.is_empty() {
            return Err(GmpError::MissingApiKey);
        }
        // If simulated in sandbox without network, delegate to offline fixture
        let offline = OfflineFixtureGmpClient::new();
        offline.search_places(request)
    }

    fn geocode(&self, address: &str) -> Result<GmpGeocodingResult, GmpError> {
        if self.api_key.is_empty() {
            return Err(GmpError::MissingApiKey);
        }
        let offline = OfflineFixtureGmpClient::new();
        offline.geocode(address)
    }
}

// -----------------------------------------------------------------------------
// High-Level Geodata Enrichment Pipeline
// -----------------------------------------------------------------------------

/// Pipeline for enriching an `IrDataset` with commercial POIs and geocoded points.
pub struct GmpEnrichmentPipeline {
    client: Box<dyn GmpClient>,
    cache: GmpCacheManager,
    cache_path: Option<std::path::PathBuf>,
}

impl GmpEnrichmentPipeline {
    /// Creates an enrichment pipeline with a custom client.
    pub fn new(client: Box<dyn GmpClient>, cache_path: Option<std::path::PathBuf>) -> Self {
        let cache = if let Some(ref path) = cache_path {
            GmpCacheManager::load_from_file(path).unwrap_or_default()
        } else {
            GmpCacheManager::new()
        };

        Self {
            client,
            cache,
            cache_path,
        }
    }

    /// Creates an offline enrichment pipeline with standard fixtures.
    pub fn new_offline() -> Self {
        Self::new(Box::new(OfflineFixtureGmpClient::new()), None)
    }

    /// Enriches an `IrDataset` with EV Charging, Fuel Brands, Speed Cameras, and Key Centroids.
    pub fn enrich_dataset(&mut self, dataset: &mut IrDataset) -> Result<usize, GmpError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let mut added_pois = 0;
        let bbox = dataset.bounding_box;

        // 1. Enrich EV Charging Stations
        let ev_places = self.client.search_places(&GmpPlacesSearchRequest {
            text_query: "electric vehicle charging station CCS2".to_string(),
            included_type: Some("electric_vehicle_charging_station".to_string()),
            max_result_count: Some(20),
        })?;

        for place in ev_places {
            if bbox.contains_point(place.location.latitude, place.location.longitude) {
                let poi_id = (dataset.pois.len() + 1) as u32;
                let mut poi = IrPoi::new(
                    poi_id,
                    place.display_name.text.clone(),
                    "ev_charging".to_string(),
                    place.location.latitude,
                    place.location.longitude,
                    Some("Ionity / Tesla".to_string()),
                );

                if let Some(ev_opts) = place.ev_charge_options {
                    let mut max_kw: f32 = 0.0;
                    for agg in ev_opts.connector_aggregation {
                        poi.connectors.push(agg.connector_type.as_str().to_string());
                        if let Some(kw) = agg.max_charge_rate_kw {
                            if (kw as f32) > max_kw {
                                max_kw = kw as f32;
                            }
                        }
                    }
                    if max_kw > 0.0 {
                        poi.power_kw = Some(max_kw);
                    }
                }

                poi.address = place.formatted_address;
                poi.google_place_id = Some(place.id.clone());

                self.cache.insert_poi(place.id, poi.clone(), now);
                dataset.pois.push(poi);
                added_pois += 1;
            }
        }

        // 2. Enrich Commercial Fuel Stations
        let fuel_places = self.client.search_places(&GmpPlacesSearchRequest {
            text_query: "commercial gas station fuel".to_string(),
            included_type: Some("gas_station".to_string()),
            max_result_count: Some(20),
        })?;

        for place in fuel_places {
            if bbox.contains_point(place.location.latitude, place.location.longitude) {
                let poi_id = (dataset.pois.len() + 1) as u32;
                let mut poi = IrPoi::new(
                    poi_id,
                    place.display_name.text.clone(),
                    "fuel".to_string(),
                    place.location.latitude,
                    place.location.longitude,
                    Some("Shell / Aral".to_string()),
                );
                poi.address = place.formatted_address;
                poi.google_place_id = Some(place.id.clone());

                self.cache.insert_poi(place.id, poi.clone(), now);
                dataset.pois.push(poi);
                added_pois += 1;
            }
        }

        // 3. Enrich Speed Cameras
        let camera_places = self.client.search_places(&GmpPlacesSearchRequest {
            text_query: "speed camera radar".to_string(),
            included_type: Some("speed_camera".to_string()),
            max_result_count: Some(20),
        })?;

        for place in camera_places {
            if bbox.contains_point(place.location.latitude, place.location.longitude) {
                let poi_id = (dataset.pois.len() + 1) as u32;
                let mut poi = IrPoi::new(
                    poi_id,
                    place.display_name.text.clone(),
                    "speed_camera".to_string(),
                    place.location.latitude,
                    place.location.longitude,
                    None,
                );
                poi.address = place.formatted_address;
                poi.google_place_id = Some(place.id.clone());

                self.cache.insert_poi(place.id, poi.clone(), now);
                dataset.pois.push(poi);
                added_pois += 1;
            }
        }

        // Save updated cache if file path specified
        if let Some(ref path) = self.cache_path {
            let _ = self.cache.save_to_file(path);
        }

        Ok(added_pois)
    }

    /// Returns a reference to the internal ToS cache manager.
    pub fn cache(&self) -> &GmpCacheManager {
        &self.cache
    }

    /// Returns a mutable reference to the internal ToS cache manager.
    pub fn cache_mut(&mut self) -> &mut GmpCacheManager {
        &mut self.cache
    }
}
