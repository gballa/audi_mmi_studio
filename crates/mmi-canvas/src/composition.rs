//! Screen Composition and Layer Hierarchy for MMI Virtual Display (800x480).

use serde::{Deserialize, Serialize};

/// Provenance of a screen layout composition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayoutProvenance {
    /// Reverse-engineered directly from firmware resources/scripts.
    Derived { evidence_tag: String },
    /// Speculatively assembled by human engineer.
    UserComposed { rationale: String },
}

/// A visual layer placed onto the screen canvas.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScreenLayer {
    pub layer_id: String,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub z_index: i32,
    pub visible: bool,
    /// Reference to asset blob in CAS
    pub asset_blob_id: Option<String>,
    /// Optional static text label
    pub text_content: Option<String>,
}

/// Screen composition representing an entire display view (e.g. Navigation, Cluster Turn-by-Turn).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScreenComposition {
    pub screen_id: String,
    pub title: String,
    pub target_width: u32,
    pub target_height: u32,
    pub provenance: LayoutProvenance,
    pub layers: Vec<ScreenLayer>,
}

impl ScreenComposition {
    /// Creates a new OEM main display canvas (default 800x480).
    pub fn new_main_display(screen_id: impl Into<String>, title: impl Into<String>, provenance: LayoutProvenance) -> Self {
        Self {
            screen_id: screen_id.into(),
            title: title.into(),
            target_width: 800,
            target_height: 480,
            provenance,
            layers: Vec::new(),
        }
    }

    /// Creates a cluster display canvas (e.g. 400x240 / 500x248).
    pub fn new_cluster_display(
        screen_id: impl Into<String>,
        title: impl Into<String>,
        width: u32,
        height: u32,
        provenance: LayoutProvenance,
    ) -> Self {
        Self {
            screen_id: screen_id.into(),
            title: title.into(),
            target_width: width,
            target_height: height,
            provenance,
            layers: Vec::new(),
        }
    }

    /// Adds a layer to the screen composition.
    pub fn add_layer(&mut self, layer: ScreenLayer) {
        self.layers.push(layer);
        self.layers.sort_by_key(|l| l.z_index);
    }
}

/// Vehicle chassis definition for MMI Drive Select 3D/Isometric models.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VehicleChassisProfile {
    pub id: String,
    pub name: String,
    pub chassis_code: String,
    pub body_type: String,
    pub wheelbase_mm: u32,
    pub default_ride_height_mm: i32,
    pub dynamic_drop_mm: i32,
}

impl VehicleChassisProfile {
    pub fn all_supported_models() -> Vec<Self> {
        vec![
            Self {
                id: "a4_sedan".to_string(),
                name: "Audi A4 3.0 TDI".to_string(),
                chassis_code: "B8.5".to_string(),
                body_type: "Sedan".to_string(),
                wheelbase_mm: 2808,
                default_ride_height_mm: 135,
                dynamic_drop_mm: -20,
            },
            Self {
                id: "a4_avant".to_string(),
                name: "Audi S4 Avant".to_string(),
                chassis_code: "B8.5".to_string(),
                body_type: "Avant".to_string(),
                wheelbase_mm: 2811,
                default_ride_height_mm: 125,
                dynamic_drop_mm: -20,
            },
            Self {
                id: "a5_coupe".to_string(),
                name: "Audi RS5 Coupe".to_string(),
                chassis_code: "8T3".to_string(),
                body_type: "Coupe".to_string(),
                wheelbase_mm: 2751,
                default_ride_height_mm: 115,
                dynamic_drop_mm: -25,
            },
            Self {
                id: "a6_allroad".to_string(),
                name: "Audi A6 allroad".to_string(),
                chassis_code: "C7".to_string(),
                body_type: "Allroad Wagon".to_string(),
                wheelbase_mm: 2905,
                default_ride_height_mm: 175,
                dynamic_drop_mm: -35,
            },
            Self {
                id: "q5_suv".to_string(),
                name: "Audi SQ5 TDI".to_string(),
                chassis_code: "8R".to_string(),
                body_type: "Performance SUV".to_string(),
                wheelbase_mm: 2813,
                default_ride_height_mm: 190,
                dynamic_drop_mm: -30,
            },
            Self {
                id: "r8_v10".to_string(),
                name: "Audi R8 V10 Plus".to_string(),
                chassis_code: "Type 42".to_string(),
                body_type: "Supercar".to_string(),
                wheelbase_mm: 2650,
                default_ride_height_mm: 105,
                dynamic_drop_mm: -15,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vehicle_chassis_profiles() {
        let models = VehicleChassisProfile::all_supported_models();
        assert_eq!(models.len(), 6);
        assert!(models.iter().any(|m| m.id == "a4_sedan"));
        assert!(models.iter().any(|m| m.id == "r8_v10"));
        assert!(models.iter().any(|m| m.id == "a6_allroad"));

        // All dynamic drops must be negative (lowering)
        for m in &models {
            assert!(m.dynamic_drop_mm < 0);
        }
    }
}
