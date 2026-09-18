//! Byte value frequency histogram and variance analysis.

use serde::{Deserialize, Serialize};

/// 256-bucket histogram mapping byte values to occurrence counts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ByteHistogram {
    pub counts: Vec<u64>,
    pub total_bytes: u64,
    pub null_byte_ratio: f64,
    pub printable_ratio: f64,
}

impl ByteHistogram {
    /// Computes the byte frequency distribution across a slice.
    pub fn compute(data: &[u8]) -> Self {
        let mut counts = vec![0u64; 256];
        let mut null_count = 0u64;
        let mut printable_count = 0u64;

        for &b in data {
            counts[b as usize] += 1;
            if b == 0 {
                null_count += 1;
            }
            if (0x20..=0x7E).contains(&b) || b == b'\n' || b == b'\r' || b == b'\t' {
                printable_count += 1;
            }
        }

        let total = data.len() as u64;
        let null_byte_ratio = if total > 0 { null_count as f64 / total as f64 } else { 0.0 };
        let printable_ratio = if total > 0 { printable_count as f64 / total as f64 } else { 0.0 };

        Self {
            counts,
            total_bytes: total,
            null_byte_ratio,
            printable_ratio,
        }
    }
}
