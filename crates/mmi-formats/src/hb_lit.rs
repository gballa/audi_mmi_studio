//! hb_lit: Binary decoder and record models for Harman/Becker LIT destination search tables.
//!
//! Conforms to Audi MMI 3G/3G+ (HN+) 544-byte FLDB physical page specifications.

use std::io::{Read, Seek, SeekFrom};
use mmi_core::CoreError;
use crate::hb_navdb::{HbNavDbHeader, FLDB_MAGIC, NAVDB_HEADER_SIZE};

pub const FLDB_PAGE_SIZE: usize = 544;
pub const FLDB_PAYLOAD_SIZE: usize = 512;
pub const FLDB_TRAILER_SYNC: u32 = 0x55AA55AA;

// Character bitmasks for MMI rotary dial speller
pub const LIT_ALPHA_MASK_A_Z: u32 = 0x03FF_FFFF; // Bits 0..25 correspond to 'A'..'Z'
pub const LIT_ALPHA_MASK_DIGITS: u32 = 1 << 26;   // Bit 26 = Digits '0'..'9'
pub const LIT_ALPHA_MASK_SPACE: u32 = 1 << 27;    // Bit 27 = Space
pub const LIT_ALPHA_MASK_SPECIAL: u32 = 1 << 28;  // Bit 28 = Diacritics/Special

/// Computes candidate letter bitmask from a slice of ASCII bytes.
pub fn compute_rotary_alpha_mask(chars: &[u8]) -> u32 {
    let mut mask = 0u32;
    for &c in chars {
        if c.is_ascii_alphabetic() {
            let offset = c.to_ascii_uppercase() - b'A';
            if offset < 26 {
                mask |= 1 << offset;
            }
        } else if c.is_ascii_digit() {
            mask |= LIT_ALPHA_MASK_DIGITS;
        } else if c == b' ' {
            mask |= LIT_ALPHA_MASK_SPACE;
        } else {
            mask |= LIT_ALPHA_MASK_SPECIAL;
        }
    }
    mask
}

/// A single branch edge in a LIT speller Radix Trie node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LitSpellerBranch {
    pub branch_char: u8,
    pub match_subcount: u16,
    pub child_page_idx: u32,
    pub entity_record_idx: u32,
}

/// In-memory representation of a Speller B-Tree node residing on a 544-byte physical page.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LitSpellerNode {
    pub page_idx: u32,
    pub valid_alpha_mask: u32,
    pub node_flags: u16,
    pub match_count: u64,
    pub branches: Vec<LitSpellerBranch>,
}

impl LitSpellerNode {
    /// Serializes node into a 512-byte payload slice.
    pub fn serialize_payload(&self) -> Vec<u8> {
        let mut buf = vec![0u8; FLDB_PAYLOAD_SIZE];
        let branch_count = (self.branches.len().min(40)) as u16;

        buf[0..2].copy_from_slice(&branch_count.to_le_bytes());
        buf[2..6].copy_from_slice(&self.valid_alpha_mask.to_le_bytes());
        buf[6..8].copy_from_slice(&self.node_flags.to_le_bytes());
        buf[8..16].copy_from_slice(&self.match_count.to_le_bytes());

        let mut offset = 16;
        for b in &self.branches[..branch_count as usize] {
            if offset + 12 > FLDB_PAYLOAD_SIZE {
                break;
            }
            buf[offset] = b.branch_char;
            buf[offset + 1] = 0; // reserved
            buf[offset + 2..offset + 4].copy_from_slice(&b.match_subcount.to_le_bytes());
            buf[offset + 4..offset + 8].copy_from_slice(&b.child_page_idx.to_le_bytes());
            buf[offset + 8..offset + 12].copy_from_slice(&b.entity_record_idx.to_le_bytes());
            offset += 12;
        }

        buf
    }

    /// Deserializes node from a 512-byte page payload.
    pub fn parse_payload(page_idx: u32, payload: &[u8]) -> Result<Self, CoreError> {
        if payload.len() < 16 {
            return Err(CoreError::ImmutabilityViolation("Speller payload too small for header".into()));
        }

        let branch_count = u16::from_le_bytes([payload[0], payload[1]]) as usize;
        let valid_alpha_mask = u32::from_le_bytes([payload[2], payload[3], payload[4], payload[5]]);
        let node_flags = u16::from_le_bytes([payload[6], payload[7]]);
        let match_count = u64::from_le_bytes([
            payload[8], payload[9], payload[10], payload[11],
            payload[12], payload[13], payload[14], payload[15],
        ]);

        let mut branches = Vec::with_capacity(branch_count);
        let mut offset = 16;

        for _ in 0..branch_count {
            if offset + 12 > payload.len() {
                break;
            }
            let branch_char = payload[offset];
            let match_subcount = u16::from_le_bytes([payload[offset + 2], payload[offset + 3]]);
            let child_page_idx = u32::from_le_bytes([
                payload[offset + 4], payload[offset + 5], payload[offset + 6], payload[offset + 7],
            ]);
            let entity_record_idx = u32::from_le_bytes([
                payload[offset + 8], payload[offset + 9], payload[offset + 10], payload[offset + 11],
            ]);

            branches.push(LitSpellerBranch {
                branch_char,
                match_subcount,
                child_page_idx,
                entity_record_idx,
            });
            offset += 12;
        }

        Ok(Self {
            page_idx,
            valid_alpha_mask,
            node_flags,
            match_count,
            branches,
        })
    }
}

/// Binary street search record stored in LIT Layer 3 (`EJ211Pa_L3.db`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LitStreetRecord {
    pub street_id: u32,
    pub city_id: u32,
    pub frc: u8,
    pub name: String,
    pub primary_edge_id: u32,
    pub centroid_x: i32,
    pub centroid_y: i32,
}

impl LitStreetRecord {
    pub const RECORD_SIZE: usize = 48;

    pub fn serialize(&self) -> [u8; Self::RECORD_SIZE] {
        let mut buf = [0u8; Self::RECORD_SIZE];
        buf[0..4].copy_from_slice(&self.street_id.to_le_bytes());
        buf[4..8].copy_from_slice(&self.city_id.to_le_bytes());
        buf[8] = self.frc;
        let name_bytes = self.name.as_bytes();
        let copy_len = name_bytes.len().min(26);
        buf[9] = copy_len as u8;
        buf[10..10 + copy_len].copy_from_slice(&name_bytes[..copy_len]);
        buf[36..40].copy_from_slice(&self.primary_edge_id.to_le_bytes());
        buf[40..44].copy_from_slice(&self.centroid_x.to_le_bytes());
        buf[44..48].copy_from_slice(&self.centroid_y.to_le_bytes());
        buf
    }

    pub fn parse(buf: &[u8; Self::RECORD_SIZE]) -> Self {
        let street_id = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
        let city_id = u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]);
        let frc = buf[8];
        let name_len = (buf[9] as usize).min(26);
        let name = String::from_utf8_lossy(&buf[10..10 + name_len]).to_string();
        let primary_edge_id = u32::from_le_bytes([buf[36], buf[37], buf[38], buf[39]]);
        let centroid_x = i32::from_le_bytes([buf[40], buf[41], buf[42], buf[43]]);
        let centroid_y = i32::from_le_bytes([buf[44], buf[45], buf[46], buf[47]]);

        Self {
            street_id,
            city_id,
            frc,
            name,
            primary_edge_id,
            centroid_x,
            centroid_y,
        }
    }
}

/// Reader for searching and traversing a physical LIT database file.
pub struct LitDatabaseReader<R> {
    reader: R,
    header: HbNavDbHeader,
}

impl<R: Read + Seek> LitDatabaseReader<R> {
    pub fn open(mut reader: R) -> Result<Self, CoreError> {
        let mut header_buf = [0u8; NAVDB_HEADER_SIZE];
        reader.read_exact(&mut header_buf).map_err(|e| CoreError::Io(e))?;
        let header = crate::hb_navdb::HbNavDb::parse(&header_buf)?.header;

        Ok(Self { reader, header })
    }

    pub fn header(&self) -> &HbNavDbHeader {
        &self.header
    }

    /// Reads a 544-byte physical page at page_idx and parses the speller node.
    pub fn read_speller_node(&mut self, page_idx: u32) -> Result<LitSpellerNode, CoreError> {
        let offset = (page_idx as u64) * (FLDB_PAGE_SIZE as u64);
        self.reader.seek(SeekFrom::Start(offset)).map_err(CoreError::Io)?;

        let mut page_buf = [0u8; FLDB_PAGE_SIZE];
        self.reader.read_exact(&mut page_buf).map_err(CoreError::Io)?;

        // Validate magic and trailer
        if &page_buf[0..4] != FLDB_MAGIC {
            return Err(CoreError::ImmutabilityViolation("Invalid page magic in LIT page".into()));
        }
        let trailer = u32::from_le_bytes([page_buf[528], page_buf[529], page_buf[530], page_buf[531]]);
        if trailer != FLDB_TRAILER_SYNC {
            return Err(CoreError::ImmutabilityViolation("Missing trailer sync in LIT page".into()));
        }

        LitSpellerNode::parse_payload(page_idx, &page_buf[16..528])
    }

    /// Traverses the speller Trie matching a search prefix, returning the candidate letter bitmask.
    pub fn get_valid_next_chars(&mut self, prefix: &str) -> Result<u32, CoreError> {
        let mut current_page = self.header.root_page + 1; // Speller root node starts on Page 2
        let prefix_upper = prefix.to_ascii_uppercase();

        for byte in prefix_upper.bytes() {
            let node = self.read_speller_node(current_page)?;
            if let Some(branch) = node.branches.iter().find(|b| b.branch_char == byte) {
                if branch.child_page_idx == 0 {
                    return Ok(0); // Terminal match with no further children
                }
                current_page = branch.child_page_idx;
            } else {
                return Ok(0); // Prefix not found
            }
        }

        let node = self.read_speller_node(current_page)?;
        Ok(node.valid_alpha_mask)
    }
}
