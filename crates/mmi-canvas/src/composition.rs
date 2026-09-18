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
