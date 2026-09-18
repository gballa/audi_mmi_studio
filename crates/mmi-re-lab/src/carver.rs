//! Signature-based container and embedded file carving detector.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CarvedFormat {
    ZlibStream,
    ElfExecutable,
    PngImage,
    ZipArchive,
    SevenZipArchive,
    QnxIfs,
    QnxEfs,
    HarmanPrecomp,
    HarmanNavDb,
    HarmanAtlas,
    HarmanAns,
    HarmanFpga,
    SmscIpf,
    HarmanGdb,
    HarmanGrammar,
    AdiLdr,
    Iso9660,
    TrueTypeFont,
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CarvedRegion {
    pub offset: usize,
    pub format: CarvedFormat,
    pub confidence: f32,
    pub header_bytes: Vec<u8>,
}

pub struct SignatureCarver;

impl SignatureCarver {
    /// Scans a byte buffer for recognizable container and file signatures.
    pub fn scan(data: &[u8]) -> Vec<CarvedRegion> {
        let mut regions = Vec::new();
        let len = data.len();

        let mut idx = 0;
        while idx < len {
            // 1. Harman Precomp (0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 6 bytes followed by w/h)
            if idx + 10 <= len && &data[idx..idx + 6] == b"\x00\x01\x00\x00\x00\x00" {
                let w = u16::from_be_bytes([data[idx + 6], data[idx + 7]]);
                let h = u16::from_be_bytes([data[idx + 8], data[idx + 9]]);
                if w > 0 && h > 0 && w < 4096 && h < 4096 {
                    regions.push(CarvedRegion {
                        offset: idx,
                        format: CarvedFormat::HarmanPrecomp,
                        confidence: 0.95,
                        header_bytes: data[idx..idx + 10].to_vec(),
                    });
                    idx += 10;
                    continue;
                }
            }

            // 1b. MapStyles Regional Archive ('rax\0', 4 bytes)
            if idx + 4 <= len && &data[idx..idx + 4] == b"rax\0" {
                regions.push(CarvedRegion {
                    offset: idx,
                    format: CarvedFormat::Unknown("HarmanMapStyleXar".to_string()),
                    confidence: 0.95,
                    header_bytes: data[idx..idx + 4].to_vec(),
                });
                idx += 4;
                continue;
            }

            // 1c. Harman/Becker NavDB FLDB (magic at offset 0x14)
            if idx == 0 && len >= 36 && &data[0x14..0x18] == b"FLDB" {
                regions.push(CarvedRegion {
                    offset: 0,
                    format: CarvedFormat::HarmanNavDb,
                    confidence: 1.0,
                    header_bytes: data[0..36].to_vec(),
                });
            }

            // 1d. Harman Orion Atlas Spatial Container
            if idx + 7 <= len && &data[idx..idx + 7] == b"\x06HEADER" {
                regions.push(CarvedRegion {
                    offset: idx,
                    format: CarvedFormat::HarmanAtlas,
                    confidence: 1.0,
                    header_bytes: data[idx..idx + 7].to_vec(),
                });
                idx += 7;
                continue;
            }

            // 1e. Harman Becker Speech Prompts (ANS\0)
            if idx + 4 <= len && &data[idx..idx + 4] == b"ANS\0" {
                regions.push(CarvedRegion {
                    offset: idx,
                    format: CarvedFormat::HarmanAns,
                    confidence: 1.0,
                    header_bytes: data[idx..idx + 4].to_vec(),
                });
                idx += 4;
                continue;
            }

            // 1f. System FPGA Bitstream (.HDG)
            if idx + 4 <= len && &data[idx..idx + 4] == b".HDG" {
                regions.push(CarvedRegion {
                    offset: idx,
                    format: CarvedFormat::HarmanFpga,
                    confidence: 1.0,
                    header_bytes: data[idx..idx + 4].to_vec(),
                });
                idx += 4;
                continue;
            }

            // 1g. SMSC MOST INIC Firmware
            if idx + 16 <= len
                && &data[idx..idx + 16]
                    == b"\x01\x0f\xff\xff\xff\xff\x01\x01\x00\x00\x20\x00\x00\x01\xdc\x00"
            {
                regions.push(CarvedRegion {
                    offset: idx,
                    format: CarvedFormat::SmscIpf,
                    confidence: 1.0,
                    header_bytes: data[idx..idx + 16].to_vec(),
                });
                idx += 16;
                continue;
            }

            // 1h. Geographic Routing Database (0xDEADBEEF)
            if idx + 4 <= len && &data[idx..idx + 4] == b"\xde\xad\xbe\xef" {
                regions.push(CarvedRegion {
                    offset: idx,
                    format: CarvedFormat::HarmanGdb,
                    confidence: 0.95,
                    header_bytes: data[idx..idx + 4].to_vec(),
                });
                idx += 4;
                continue;
            }

            // 1i. Harman Becker Binary Grammar (0xFFFFFFFE)
            if idx + 4 <= len && &data[idx..idx + 4] == b"\xfe\xff\xff\xff" {
                regions.push(CarvedRegion {
                    offset: idx,
                    format: CarvedFormat::HarmanGrammar,
                    confidence: 0.95,
                    header_bytes: data[idx..idx + 4].to_vec(),
                });
                idx += 4;
                continue;
            }

            // 1j. Analog Devices Blackfin DSP Loader (0xB8C6D3E2)
            if idx + 4 <= len && &data[idx..idx + 4] == b"\xe2\xd3\xc6\xb8" {
                regions.push(CarvedRegion {
                    offset: idx,
                    format: CarvedFormat::AdiLdr,
                    confidence: 1.0,
                    header_bytes: data[idx..idx + 4].to_vec(),
                });
                idx += 4;
                continue;
            }

            // 2. PNG Bitmap ('\x89PNG\r\n\x1a\n', 8 bytes)
            if idx + 8 <= len && &data[idx..idx + 8] == b"\x89PNG\r\n\x1a\n" {
                regions.push(CarvedRegion {
                    offset: idx,
                    format: CarvedFormat::PngImage,
                    confidence: 1.0,
                    header_bytes: data[idx..idx + 8].to_vec(),
                });
                idx += 8;
                continue;
            }

            // 3. ELF Binary ('\x7fELF', 4 bytes)
            if idx + 4 <= len && &data[idx..idx + 4] == b"\x7fELF" {
                regions.push(CarvedRegion {
                    offset: idx,
                    format: CarvedFormat::ElfExecutable,
                    confidence: 0.95,
                    header_bytes: data[idx..idx + 4].to_vec(),
                });
                idx += 4;
                continue;
            }

            // 4. ZIP Archive ('PK\x03\x04', 4 bytes)
            if idx + 4 <= len && &data[idx..idx + 4] == b"PK\x03\x04" {
                regions.push(CarvedRegion {
                    offset: idx,
                    format: CarvedFormat::ZipArchive,
                    confidence: 0.95,
                    header_bytes: data[idx..idx + 4].to_vec(),
                });
                idx += 4;
                continue;
            }

            // 5. 7-Zip Container ('7z\xbc\xaf\x27\x1c', 6 bytes)
            if idx + 6 <= len && &data[idx..idx + 6] == b"7z\xbc\xaf\x27\x1c" {
                regions.push(CarvedRegion {
                    offset: idx,
                    format: CarvedFormat::SevenZipArchive,
                    confidence: 1.0,
                    header_bytes: data[idx..idx + 6].to_vec(),
                });
                idx += 6;
                continue;
            }

            // 6. QNX Image FileSystem ('\xeb\x7e\xff\x00' or '\x00\xff\x7e\xeb', 4 bytes)
            if idx + 4 <= len
                && (&data[idx..idx + 4] == b"\xeb\x7e\xff\x00"
                    || &data[idx..idx + 4] == b"\x00\xff\x7e\xeb")
            {
                regions.push(CarvedRegion {
                    offset: idx,
                    format: CarvedFormat::QnxIfs,
                    confidence: 0.9,
                    header_bytes: data[idx..idx + 4].to_vec(),
                });
                idx += 4;
                continue;
            }

            // 6b. QNX Flash FileSystem (QSSL_F3S at offset 0x2C)
            if idx + 8 <= len && &data[idx..idx + 8] == b"QSSL_F3S" {
                regions.push(CarvedRegion {
                    offset: idx,
                    format: CarvedFormat::QnxEfs,
                    confidence: 1.0,
                    header_bytes: data[idx..idx + 8].to_vec(),
                });
                idx += 8;
                continue;
            }

            // 7. TrueType Font ('\x00\x01\x00\x00' or 'true', 4 bytes)
            if idx + 4 <= len
                && (&data[idx..idx + 4] == b"\x00\x01\x00\x00" || &data[idx..idx + 4] == b"true")
            {
                regions.push(CarvedRegion {
                    offset: idx,
                    format: CarvedFormat::TrueTypeFont,
                    confidence: 0.85,
                    header_bytes: data[idx..idx + 4].to_vec(),
                });
                idx += 4;
                continue;
            }

            // 8. zlib stream (0x7801, 0x789C, 0x78DA, 0x785E)
            if idx + 2 <= len
                && data[idx] == 0x78
                && matches!(data[idx + 1], 0x01 | 0x5E | 0x9C | 0xDA)
            {
                // Confirm valid checksum check: (b0 * 256 + b1) % 31 == 0
                let val = (data[idx] as u16) * 256 + (data[idx + 1] as u16);
                if val % 31 == 0 {
                    regions.push(CarvedRegion {
                        offset: idx,
                        format: CarvedFormat::ZlibStream,
                        confidence: 0.8,
                        header_bytes: data[idx..idx + 2].to_vec(),
                    });
                    idx += 2;
                    continue;
                }
            }

            idx += 1;
        }

        regions
    }
}
