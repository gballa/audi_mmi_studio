//! Shannon entropy calculation and region classification.

use serde::{Deserialize, Serialize};

/// Classification of a byte span based on its Shannon entropy value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntropyClass {
    /// Plaintext or zero-filled data (H < 4.5)
    Plaintext,
    /// Structured records, code, or sparse binary data (4.5 <= H < 7.2)
    Structured,
    /// Standard compressed data (e.g. zlib, deflate, lzma) (7.2 <= H < 7.95)
    Compressed,
    /// Encrypted or maximum-entropy pseudo-random bytes (H >= 7.95)
    EncryptedOrRandom,
}

impl EntropyClass {
    pub fn from_entropy(h: f64) -> Self {
        if h < 4.5 {
            EntropyClass::Plaintext
        } else if h < 7.2 {
            EntropyClass::Structured
        } else if h < 7.95 {
            EntropyClass::Compressed
        } else {
            EntropyClass::EncryptedOrRandom
        }
    }
}

/// A contiguous span of bytes with an associated Shannon entropy score.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntropySegment {
    pub offset: usize,
    pub length: usize,
    pub entropy: f64,
    pub classification: EntropyClass,
}

pub struct EntropyCalculator;

impl EntropyCalculator {
    /// Computes the Shannon entropy of a slice: H = -sum(p * log2(p)).
    /// Resulting value is in range [0.0, 8.0].
    pub fn shannon_entropy(data: &[u8]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }

        let mut freq = [0u64; 256];
        for &b in data {
            freq[b as usize] += 1;
        }

        let len_f = data.len() as f64;
        let mut entropy = 0.0;

        for &count in &freq {
            if count > 0 {
                let p = count as f64 / len_f;
                entropy -= p * p.log2();
            }
        }

        entropy
    }

    /// Computes a strip of entropy segments across the buffer using fixed-size windows.
    pub fn compute_strip(data: &[u8], window_size: usize) -> Vec<EntropySegment> {
        let win = if window_size == 0 { 1024 } else { window_size };
        let mut segments = Vec::new();

        for (i, chunk) in data.chunks(win).enumerate() {
            let offset = i * win;
            let h = Self::shannon_entropy(chunk);
            segments.push(EntropySegment {
                offset,
                length: chunk.len(),
                entropy: h,
                classification: EntropyClass::from_entropy(h),
            });
        }

        segments
    }
}
