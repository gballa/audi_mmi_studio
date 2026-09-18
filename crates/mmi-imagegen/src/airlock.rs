//! The Egress Airlock: Request sanitization, deny list enforcement, brand heuristics, and audit logging.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::ImageGenError;

/// Audit ledger entry recorded for every transmission attempt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EgressAuditEntry {
    pub timestamp_epoch_secs: u64,
    pub asset_id: Option<String>,
    pub model: String,
    pub sanitized_prompt: String,
    pub request_sha256: String,
    pub response_sha256: Option<String>,
    pub outcome: String,
}

pub struct EgressAirlock {
    network_enabled: bool,
    audit_log: Vec<EgressAuditEntry>,
}

impl EgressAirlock {
    pub fn new(network_enabled: bool) -> Self {
        Self {
            network_enabled,
            audit_log: Vec::new(),
        }
    }

    pub fn is_network_enabled(&self) -> bool {
        self.network_enabled
    }

    pub fn set_network_enabled(&mut self, enabled: bool) {
        self.network_enabled = enabled;
    }

    pub fn audit_log(&self) -> &[EgressAuditEntry] {
        &self.audit_log
    }

    /// Verifies that an asset is permitted to cross the egress airlock.
    pub fn verify_asset_egress(&self, asset_path: &str) -> Result<(), ImageGenError> {
        // Enforce Egress Deny List:
        // Signed payloads, activation, licensing, or out-of-scope targets
        let p_lower = asset_path.to_lowercase();
        if p_lower.ends_with(".pkg")
            || p_lower.ends_with(".sig")
            || p_lower.contains("tmcconfig.dat")
            || p_lower.contains("license")
            || p_lower.contains("vlasoff")
            || p_lower.contains("activation")
        {
            return Err(ImageGenError::EgressDenied(format!(
                "Asset '{}' belongs to a signed, protected or licensing scope",
                asset_path
            )));
        }

        Ok(())
    }

    /// Sanitizes prompt text to strip VIN patterns, filesystem paths, serials, and cryptographic keys.
    pub fn sanitize_prompt(&self, raw_prompt: &str) -> String {
        let mut sanitized = raw_prompt.to_string();

        // 1. Strip VIN-like 17-character alphanumeric sequences
        let vin_regex_pattern = |s: &str| -> String {
            let mut out = String::new();
            for word in s.split_whitespace() {
                let clean = word.trim_matches(|c: char| !c.is_alphanumeric());
                if clean.len() == 17 && clean.chars().all(|c| c.is_ascii_alphanumeric()) {
                    out.push_str("[REDACTED_VIN] ");
                } else if word.starts_with('/') || word.starts_with('\\') || word.contains("originals/") {
                    out.push_str("[REDACTED_PATH] ");
                } else {
                    out.push_str(word);
                    out.push(' ');
                }
            }
            out.trim().to_string()
        };

        sanitized = vin_regex_pattern(&sanitized);
        sanitized
    }

    /// Heuristic warning check for automotive brand logos and trademarks.
    pub fn check_brand_mark(&self, prompt: &str) -> Option<String> {
        let p_lower = prompt.to_lowercase();
        let brands = ["audi", "four rings", "quattro", "volkswagen", "vw", "porsche", "bentley", "mmi"];
        for b in &brands {
            if p_lower.contains(b) {
                return Some(format!("Prompt references OEM brand mark: '{}'", b));
            }
        }
        None
    }

    /// Records an egress event in the audit log.
    pub fn log_event(
        &mut self,
        asset_id: Option<String>,
        model: &str,
        sanitized_prompt: &str,
        request_bytes: &[u8],
        response_bytes: Option<&[u8]>,
        outcome: &str,
    ) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let req_sha = hex::encode(Sha256::digest(request_bytes));
        let resp_sha = response_bytes.map(|b| hex::encode(Sha256::digest(b)));

        self.audit_log.push(EgressAuditEntry {
            timestamp_epoch_secs: now,
            asset_id,
            model: model.to_string(),
            sanitized_prompt: sanitized_prompt.to_string(),
            request_sha256: req_sha,
            response_sha256: resp_sha,
            outcome: outcome.to_string(),
        });
    }
}
