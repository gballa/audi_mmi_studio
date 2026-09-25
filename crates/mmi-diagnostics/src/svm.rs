use serde::{Deserialize, Serialize};
use crate::can::CanAdapter;
use crate::error::DiagnosticsError;
use crate::isotp::IsoTpConfig;
use crate::uds::*;

pub const SVM_CHANNEL_15_XOR_KEY: u16 = 51666; // 0xC9D2
pub const SVM_CHANNEL_15_XOR_CIPHER: u32 = 51666;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SvmResolutionResult {
    pub connected: bool,
    pub original_channel_15: u16,
    pub updated_channel_15: u16,
    pub dtc_03276_cleared: bool,
    pub dtc_03175_cleared: bool,
    pub gem_unlocked: bool,
    pub details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Svm03175Report {
    pub rehash_executed: bool,
    pub initial_value: u16,
    pub toggled_value: u16,
    pub restored_value: u16,
    pub dtc_03175_cleared: bool,
    pub details: String,
}

pub struct SvmSolver;

impl SvmSolver {
    /// Pure mathematical function to compute Channel 15 XOR response (u16)
    pub fn calculate_channel_15_xor(current_val: u16) -> u16 {
        current_val ^ SVM_CHANNEL_15_XOR_KEY
    }

    /// Pure mathematical function to solve SVM 03276 challenge (u32)
    pub fn solve_svm_03276(challenge: u32) -> u32 {
        challenge ^ SVM_CHANNEL_15_XOR_CIPHER
    }

    /// Pure mathematical simulation of SVM 03175 +1/-1 parameter rehash
    pub fn simulate_svm_03175_rehash(initial_setting: u32) -> (u32, u32, bool) {
        let stage1 = initial_setting + 1;
        let stage2 = stage1 - 1;
        (stage1, stage2, stage2 == initial_setting)
    }

    /// Executes full automated SVM 03276 fault resolution over UDS
    pub fn resolve_svm(
        adapter: &mut dyn CanAdapter,
        enable_gem: bool,
    ) -> Result<SvmResolutionResult, DiagnosticsError> {
        let mut client = UdsClient::new(adapter, IsoTpConfig::default());

        // 1. Enter Extended Diagnostic Session (0x10 0x03)
        let _ = client.session_control(SESSION_EXTENDED);

        // 2. Read Adaptation Channel 15 (DID 0x0615)
        let ch15_raw = client.read_did(DID_ADAPTATION_CHANNEL_15)?;
        let original_val = if ch15_raw.len() >= 2 {
            u16::from_be_bytes([ch15_raw[0], ch15_raw[1]])
        } else {
            0
        };

        // 3. Calculate XOR Key and write updated value
        let updated_val = Self::calculate_channel_15_xor(original_val);
        client.write_did(DID_ADAPTATION_CHANNEL_15, &updated_val.to_be_bytes())?;

        // 4. Verify write by reading back Channel 15
        let ch15_verify = client.read_did(DID_ADAPTATION_CHANNEL_15)?;
        let verified_val = if ch15_verify.len() >= 2 {
            u16::from_be_bytes([ch15_verify[0], ch15_verify[1]])
        } else {
            0
        };
        if verified_val != updated_val {
            return Err(DiagnosticsError::InvalidResponse(format!(
                "Channel 15 write verification failed: expected {}, read {}",
                updated_val, verified_val
            )));
        }

        // 5. If requested, enable Green Engineering Menu (DID 0x0606 or 0x0611 = 1)
        let mut gem_unlocked = false;
        if enable_gem {
            let _ = client.write_did(DID_CODING, &[0x01]);
            if client.write_did(DID_GREEN_MENU_ENABLE, &[0x01]).is_ok() {
                gem_unlocked = true;
            }
        }

        // 6. Clear DTCs via service 0x14
        client.clear_dtcs(0xFFFFFF)?;

        Ok(SvmResolutionResult {
            connected: true,
            original_channel_15: original_val,
            updated_channel_15: updated_val,
            dtc_03276_cleared: true,
            dtc_03175_cleared: true,
            gem_unlocked,
            details: format!(
                "Successfully resolved SVM 03276 (Channel 15: {} -> {}) and cleared DTCs",
                original_val, updated_val
            ),
        })
    }

    /// Executes automated SVM 03175 resolution via Car Menu parameter rehash (+1 then -1 toggle)
    pub fn resolve_svm_03175(adapter: &mut dyn CanAdapter) -> Result<Svm03175Report, DiagnosticsError> {
        let mut client = UdsClient::new(adapter, IsoTpConfig::default());

        // 1. Enter Extended Diagnostic Session (0x10 0x03)
        let _ = client.session_control(SESSION_EXTENDED);

        // 2. Read Car Menu Configuration setting (DID 0x0620 / Channel 32)
        let raw_setting = client.read_did(DID_CAR_MENU_CONFIG)?;
        let initial_val = if raw_setting.len() >= 2 {
            u16::from_be_bytes([raw_setting[0], raw_setting[1]])
        } else {
            5 // fallback standard baseline
        };

        // 3. Stage 1: write V + 1 to trigger config hash invalidation
        let toggled_val = initial_val.wrapping_add(1);
        client.write_did(DID_CAR_MENU_CONFIG, &toggled_val.to_be_bytes())?;

        // 4. Stage 2: write back original V to commit valid persistent configuration
        client.write_did(DID_CAR_MENU_CONFIG, &initial_val.to_be_bytes())?;

        // 5. Clear DTCs via service 0x14
        client.clear_dtcs(0xFFFFFF)?;

        Ok(Svm03175Report {
            rehash_executed: true,
            initial_value: initial_val,
            toggled_value: toggled_val,
            restored_value: initial_val,
            dtc_03175_cleared: true,
            details: format!(
                "Successfully rehashed dataset configuration (+1/-1 toggle on DID 0x0620: {} -> {} -> {}) and cleared DTC 03175",
                initial_val, toggled_val, initial_val
            ),
        })
    }
}
