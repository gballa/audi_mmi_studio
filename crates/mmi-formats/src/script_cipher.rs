//! script_cipher.rs: PRNG-based symmetric XOR cipher for Audi MMI 3G/3G+ copie_scr.sh.
//!
//! Reimplementation of the Harman-Becker PRNG cipher used by QNX `proc_scriptlauncher`
//! to authenticate and execute SD card autorun scripts.
//!
//! Seed constant: `0x001be3ac`.
//!
//! The cipher is symmetric: applying the cipher twice reproduces the original plaintext.

pub const SEED_INIT: u32 = 0x001be3ac;

/// PRNG-based XOR cipher matching Harman-Becker's MMI 3G/3G+ `proc_scriptlauncher`.
#[derive(Debug, Clone)]
pub struct Mmi3gScriptCipher {
    seed: u32,
}

impl Default for Mmi3gScriptCipher {
    fn default() -> Self {
        Self { seed: SEED_INIT }
    }
}

impl Mmi3gScriptCipher {
    pub fn new() -> Self {
        Self::default()
    }

    /// Generates next PRNG 32-bit state value matching the Harman-Becker SH-4 implementation.
    #[inline]
    fn prng_rand(&mut self) -> u32 {
        let r0 = self.seed;
        let r1 = (self.seed >> 1) | (self.seed << 31);
        let r3 = ((r1 >> 16) & 0xFF).wrapping_add(r1);
        let r1_new = ((r3 >> 8) & 0xFF) << 16;
        let r3_new = r3.wrapping_sub(r1_new);
        self.seed = r3_new;
        r0
    }

    /// Encodes or decodes a byte slice. Because this is a stream XOR cipher,
    /// encode and decode are identical symmetric operations.
    pub fn process(&mut self, data: &[u8]) -> Vec<u8> {
        self.seed = SEED_INIT;
        // The first PRNG value is discarded in the Harman-Becker algorithm
        let _ = self.prng_rand();

        let mut output = Vec::with_capacity(data.len());
        for &byte in data {
            let xor_val = (self.prng_rand() & 0xFF) as u8;
            output.push(byte ^ xor_val);
        }
        output
    }

    /// Convenience static method to process data with a fresh cipher state.
    pub fn transform(data: &[u8]) -> Vec<u8> {
        let mut cipher = Self::new();
        cipher.process(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_script_cipher_roundtrip_symmetry() {
        let plaintext = b"#!/bin/sh\necho \"Audi MMI 3G+ Test Payload\"\nmount -uw /mnt/efs-system\nexit 0\n";
        let encoded = Mmi3gScriptCipher::transform(plaintext);
        assert_ne!(encoded, plaintext);
        assert_eq!(encoded.len(), plaintext.len());

        let decoded = Mmi3gScriptCipher::transform(&encoded);
        assert_eq!(decoded, plaintext);
    }

    #[test]
    fn test_script_cipher_empty_data() {
        let empty = b"";
        let transformed = Mmi3gScriptCipher::transform(empty);
        assert!(transformed.is_empty());
    }

    #[test]
    fn test_script_cipher_matches_known_prng_stream() {
        let mut cipher = Mmi3gScriptCipher::new();
        // Discard first
        let _ = cipher.prng_rand();
        let k0 = (cipher.prng_rand() & 0xFF) as u8;
        let k1 = (cipher.prng_rand() & 0xFF) as u8;
        let k2 = (cipher.prng_rand() & 0xFF) as u8;

        let input = [0x00, 0x00, 0x00];
        let out = Mmi3gScriptCipher::transform(&input);
        assert_eq!(out, [k0, k1, k2]);
    }
}
