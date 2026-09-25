//! Harman Becker Geographic Routing Database (GDB) format parser & serializer (§5, RQ-010).
//! Conforms to Audi MMI 3G+ (HN+) 544-byte physical page stride with CRC-16/CCITT.

use std::io::{Seek, SeekFrom, Write};
use std::path::Path;

use crate::adapter::{FormatAdapter, FormatCapabilities};
use mmi_core::CoreError;
use serde::{Deserialize, Serialize};

pub const GDB_MAGIC: &[u8; 4] = &[0xde, 0xad, 0xbe, 0xef];
pub const GDB_VERSION_37: u32 = 37;

pub const GDB_PAGE_SIZE: usize = 544;
pub const GDB_PAYLOAD_SIZE: usize = 512;
pub const GDB_HEADER_SIZE: usize = 16;
pub const GDB_TRAILER_SIZE: usize = 16;
pub const GDB_TRAILER_SYNC: u32 = 0x55AA55AA;

pub type HbGdbHeader = GdbMasterHeader;

/// CRC-16/CCITT polynomial 0x1021 with seed 0xFFFF (AUTOSAR standard)
pub fn gdb_crc16_ccitt(data: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;
    for &b in data {
        crc ^= (b as u16) << 8;
        for _ in 0..8 {
            if (crc & 0x8000) != 0 {
                crc = (crc << 1) ^ 0x1021;
            } else {
                crc <<= 1;
            }
        }
    }
    crc
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GdbMasterHeader {
    pub magic: [u8; 4],
    pub version: u32,
    pub flags: u32,
    pub total_pages: u32,
    pub node_count: u32,
    pub edge_count: u32,
    pub restriction_count: u32,
    pub morton_root_page: u32,
    pub frc_layer_offsets: [u32; 8], // Layer root page pointers for FRC 0..7
    pub page_size: u32,
    pub morton_tile_count: u32,
    pub frc_layer_counts: [u32; 8],
}

impl Default for GdbMasterHeader {
    fn default() -> Self {
        Self {
            magic: *GDB_MAGIC,
            version: GDB_VERSION_37,
            flags: 0,
            total_pages: 1,
            node_count: 0,
            edge_count: 0,
            restriction_count: 0,
            morton_root_page: 0,
            frc_layer_offsets: [0; 8],
            page_size: GDB_PAGE_SIZE as u32,
            morton_tile_count: 0,
            frc_layer_counts: [0; 8],
        }
    }
}

impl GdbMasterHeader {
    pub fn parse(payload: &[u8]) -> Result<Self, CoreError> {
        if payload.len() < 64 {
            return Err(CoreError::ImmutabilityViolation(
                "GDB master payload too small for header".into(),
            ));
        }

        let mut magic = [0u8; 4];
        magic.copy_from_slice(&payload[0..4]);
        if &magic != GDB_MAGIC {
            return Err(CoreError::ImmutabilityViolation(format!(
                "Invalid GDB magic: expected 0xDEADBEEF, found {:02X?}",
                magic
            )));
        }

        let version = u32::from_be_bytes([payload[4], payload[5], payload[6], payload[7]]);
        let flags = u32::from_be_bytes([payload[8], payload[9], payload[10], payload[11]]);
        let total_pages = u32::from_be_bytes([payload[12], payload[13], payload[14], payload[15]]);
        let node_count = u32::from_be_bytes([payload[16], payload[17], payload[18], payload[19]]);
        let edge_count = u32::from_be_bytes([payload[20], payload[21], payload[22], payload[23]]);
        let restriction_count = u32::from_be_bytes([payload[24], payload[25], payload[26], payload[27]]);
        let morton_root_page = u32::from_be_bytes([payload[28], payload[29], payload[30], payload[31]]);

        let mut frc_layer_offsets = [0u32; 8];
        for i in 0..8 {
            let offset = 32 + i * 4;
            frc_layer_offsets[i] = u32::from_be_bytes([
                payload[offset],
                payload[offset + 1],
                payload[offset + 2],
                payload[offset + 3],
            ]);
        }

        let (page_size, morton_tile_count, frc_layer_counts) = if payload.len() >= 104 {
            let ps = u32::from_be_bytes([payload[64], payload[65], payload[66], payload[67]]);
            let mtc = u32::from_be_bytes([payload[68], payload[69], payload[70], payload[71]]);
            let mut flc = [0u32; 8];
            for i in 0..8 {
                let off = 72 + i * 4;
                flc[i] = u32::from_be_bytes([
                    payload[off],
                    payload[off + 1],
                    payload[off + 2],
                    payload[off + 3],
                ]);
            }
            (ps, mtc, flc)
        } else {
            (GDB_PAGE_SIZE as u32, 0, [0u32; 8])
        };

        Ok(Self {
            magic,
            version,
            flags,
            total_pages,
            node_count,
            edge_count,
            restriction_count,
            morton_root_page,
            frc_layer_offsets,
            page_size,
            morton_tile_count,
            frc_layer_counts,
        })
    }

    pub fn serialize_payload(&self) -> Vec<u8> {
        let mut buf = vec![0u8; GDB_PAYLOAD_SIZE];
        buf[0..4].copy_from_slice(&self.magic);
        buf[4..8].copy_from_slice(&self.version.to_be_bytes());
        buf[8..12].copy_from_slice(&self.flags.to_be_bytes());
        buf[12..16].copy_from_slice(&self.total_pages.to_be_bytes());
        buf[16..20].copy_from_slice(&self.node_count.to_be_bytes());
        buf[20..24].copy_from_slice(&self.edge_count.to_be_bytes());
        buf[24..28].copy_from_slice(&self.restriction_count.to_be_bytes());
        buf[28..32].copy_from_slice(&self.morton_root_page.to_be_bytes());

        for (i, &offset) in self.frc_layer_offsets.iter().enumerate() {
            let idx = 32 + i * 4;
            buf[idx..idx + 4].copy_from_slice(&offset.to_be_bytes());
        }

        buf[64..68].copy_from_slice(&self.page_size.to_be_bytes());
        buf[68..72].copy_from_slice(&self.morton_tile_count.to_be_bytes());
        for (i, &count) in self.frc_layer_counts.iter().enumerate() {
            let idx = 72 + i * 4;
            buf[idx..idx + 4].copy_from_slice(&count.to_be_bytes());
        }

        buf
    }
}

/// Routing Node Record (16 bytes per node)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GdbNodeRecord {
    pub node_id: u32,
    pub lat_fixed: i32,
    pub lon_fixed: i32,
    pub junction_flags: u16,
    pub reserved: u16,
}

impl GdbNodeRecord {
    pub fn new(
        node_id: u32,
        lat_fixed: i32,
        lon_fixed: i32,
        junction_flags: u16,
        morton_tile_id: u16,
    ) -> Self {
        Self {
            node_id,
            lat_fixed,
            lon_fixed,
            junction_flags,
            reserved: morton_tile_id,
        }
    }

    pub fn morton_tile_id(&self) -> u16 {
        self.reserved
    }

    pub fn serialize(&self) -> [u8; 16] {
        let mut b = [0u8; 16];
        b[0..4].copy_from_slice(&self.node_id.to_be_bytes());
        b[4..8].copy_from_slice(&self.lat_fixed.to_be_bytes());
        b[8..12].copy_from_slice(&self.lon_fixed.to_be_bytes());
        b[12..14].copy_from_slice(&self.junction_flags.to_be_bytes());
        b[14..16].copy_from_slice(&self.reserved.to_be_bytes());
        b
    }

    pub fn parse(b: &[u8]) -> Self {
        let node_id = u32::from_be_bytes([b[0], b[1], b[2], b[3]]);
        let lat_fixed = i32::from_be_bytes([b[4], b[5], b[6], b[7]]);
        let lon_fixed = i32::from_be_bytes([b[8], b[9], b[10], b[11]]);
        let junction_flags = u16::from_be_bytes([b[12], b[13]]);
        let reserved = u16::from_be_bytes([b[14], b[15]]);
        Self {
            node_id,
            lat_fixed,
            lon_fixed,
            junction_flags,
            reserved,
        }
    }
}

/// Routing Edge/Link Record (24 bytes per edge)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GdbEdgeRecord {
    pub edge_id: u32,
    pub from_node: u32,
    pub to_node: u32,
    pub length_dm: u32,
    pub frc: u8,
    pub speed_forward: u8,
    pub speed_backward: u8,
    pub lanes: u8,
    pub turn_lanes_mask: u16,
    pub access_flags: u16,
}

impl GdbEdgeRecord {
    pub fn serialize(&self) -> [u8; 24] {
        let mut b = [0u8; 24];
        b[0..4].copy_from_slice(&self.edge_id.to_be_bytes());
        b[4..8].copy_from_slice(&self.from_node.to_be_bytes());
        b[8..12].copy_from_slice(&self.to_node.to_be_bytes());
        b[12..16].copy_from_slice(&self.length_dm.to_be_bytes());
        b[16] = self.frc;
        b[17] = self.speed_forward;
        b[18] = self.speed_backward;
        b[19] = self.lanes;
        b[20..22].copy_from_slice(&self.turn_lanes_mask.to_be_bytes());
        b[22..24].copy_from_slice(&self.access_flags.to_be_bytes());
        b
    }

    pub fn parse(b: &[u8]) -> Self {
        let edge_id = u32::from_be_bytes([b[0], b[1], b[2], b[3]]);
        let from_node = u32::from_be_bytes([b[4], b[5], b[6], b[7]]);
        let to_node = u32::from_be_bytes([b[8], b[9], b[10], b[11]]);
        let length_dm = u32::from_be_bytes([b[12], b[13], b[14], b[15]]);
        let frc = b[16];
        let speed_forward = b[17];
        let speed_backward = b[18];
        let lanes = b[19];
        let turn_lanes_mask = u16::from_be_bytes([b[20], b[21]]);
        let access_flags = u16::from_be_bytes([b[22], b[23]]);
        Self {
            edge_id,
            from_node,
            to_node,
            length_dm,
            frc,
            speed_forward,
            speed_backward,
            lanes,
            turn_lanes_mask,
            access_flags,
        }
    }
}

/// Turn Restriction Record (24 bytes)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GdbTurnRestrictionRecord {
    pub restriction_id: u32,
    pub from_edge: u32,
    pub via_node: u32,
    pub to_edge: u32,
    pub restriction_type: u16,
    pub penalty_s: u16,
    pub reserved: u32,
}

impl GdbTurnRestrictionRecord {
    pub fn new(
        restriction_id: u32,
        from_edge: u32,
        via_node: u32,
        to_edge: u32,
        restriction_type: u16,
        penalty_s: u16,
    ) -> Self {
        Self {
            restriction_id,
            from_edge,
            via_node,
            to_edge,
            restriction_type,
            penalty_s,
            reserved: 0,
        }
    }

    pub fn serialize(&self) -> [u8; 24] {
        let mut b = [0u8; 24];
        b[0..4].copy_from_slice(&self.restriction_id.to_be_bytes());
        b[4..8].copy_from_slice(&self.from_edge.to_be_bytes());
        b[8..12].copy_from_slice(&self.via_node.to_be_bytes());
        b[12..16].copy_from_slice(&self.to_edge.to_be_bytes());
        b[16..18].copy_from_slice(&self.restriction_type.to_be_bytes());
        b[18..20].copy_from_slice(&self.penalty_s.to_be_bytes());
        b[20..24].copy_from_slice(&self.reserved.to_be_bytes());
        b
    }

    pub fn parse(b: &[u8]) -> Self {
        let restriction_id = u32::from_be_bytes([b[0], b[1], b[2], b[3]]);
        let from_edge = u32::from_be_bytes([b[4], b[5], b[6], b[7]]);
        let via_node = u32::from_be_bytes([b[8], b[9], b[10], b[11]]);
        let to_edge = u32::from_be_bytes([b[12], b[13], b[14], b[15]]);
        let (restriction_type, penalty_s, reserved) = if b.len() >= 24 {
            let rt = u16::from_be_bytes([b[16], b[17]]);
            let pen = u16::from_be_bytes([b[18], b[19]]);
            let res = u32::from_be_bytes([b[20], b[21], b[22], b[23]]);
            (rt, pen, res)
        } else if b.len() >= 20 {
            let rt = u16::from_be_bytes([b[16], b[17]]);
            let pen = u16::from_be_bytes([b[18], b[19]]);
            (rt, pen, 0)
        } else {
            (1, 0xFFFF, 0)
        };
        Self {
            restriction_id,
            from_edge,
            via_node,
            to_edge,
            restriction_type,
            penalty_s,
            reserved,
        }
    }
}

/// Morton Z-curve Spatial Tile Record (48 bytes)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GdbSpatialTileRecord {
    pub morton_key: u64,
    pub tile_id: u32,
    pub start_node_idx: u32,
    pub node_count: u32,
    pub start_edge_idx: u32,
    pub edge_count: u32,
    pub min_lat: i32,
    pub min_lon: i32,
    pub max_lat: i32,
    pub max_lon: i32,
    pub reserved: u32,
}

impl GdbSpatialTileRecord {
    pub fn serialize(&self) -> [u8; 48] {
        let mut b = [0u8; 48];
        b[0..8].copy_from_slice(&self.morton_key.to_be_bytes());
        b[8..12].copy_from_slice(&self.tile_id.to_be_bytes());
        b[12..16].copy_from_slice(&self.start_node_idx.to_be_bytes());
        b[16..20].copy_from_slice(&self.node_count.to_be_bytes());
        b[20..24].copy_from_slice(&self.start_edge_idx.to_be_bytes());
        b[24..28].copy_from_slice(&self.edge_count.to_be_bytes());
        b[28..32].copy_from_slice(&self.min_lat.to_be_bytes());
        b[32..36].copy_from_slice(&self.min_lon.to_be_bytes());
        b[36..40].copy_from_slice(&self.max_lat.to_be_bytes());
        b[40..44].copy_from_slice(&self.max_lon.to_be_bytes());
        b[44..48].copy_from_slice(&self.reserved.to_be_bytes());
        b
    }

    pub fn parse(b: &[u8]) -> Self {
        let morton_key = u64::from_be_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]);
        let tile_id = u32::from_be_bytes([b[8], b[9], b[10], b[11]]);
        let start_node_idx = u32::from_be_bytes([b[12], b[13], b[14], b[15]]);
        let node_count = u32::from_be_bytes([b[16], b[17], b[18], b[19]]);
        let start_edge_idx = u32::from_be_bytes([b[20], b[21], b[22], b[23]]);
        let edge_count = u32::from_be_bytes([b[24], b[25], b[26], b[27]]);
        let min_lat = i32::from_be_bytes([b[28], b[29], b[30], b[31]]);
        let min_lon = i32::from_be_bytes([b[32], b[33], b[34], b[35]]);
        let max_lat = i32::from_be_bytes([b[36], b[37], b[38], b[39]]);
        let max_lon = i32::from_be_bytes([b[40], b[41], b[42], b[43]]);
        let reserved = if b.len() >= 48 {
            u32::from_be_bytes([b[44], b[45], b[46], b[47]])
        } else {
            0
        };
        Self {
            morton_key,
            tile_id,
            start_node_idx,
            node_count,
            start_edge_idx,
            edge_count,
            min_lat,
            min_lon,
            max_lat,
            max_lon,
            reserved,
        }
    }
}

/// 16-byte physical page header conforming to Harman/Becker GDB specification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GdbPageHeader {
    pub page_id: u32,
    pub crc16: u16,
    pub flags: u16,
    pub payload_len: u16,
    pub reserved: [u8; 6],
}

impl GdbPageHeader {
    pub fn serialize(&self) -> [u8; 16] {
        let mut b = [0u8; 16];
        b[0..4].copy_from_slice(&self.page_id.to_be_bytes());
        b[4..6].copy_from_slice(&self.crc16.to_be_bytes());
        b[6..8].copy_from_slice(&self.flags.to_be_bytes());
        b[8..10].copy_from_slice(&self.payload_len.to_be_bytes());
        b[10..16].copy_from_slice(&self.reserved);
        b
    }

    pub fn parse(b: &[u8]) -> Self {
        let page_id = u32::from_be_bytes([b[0], b[1], b[2], b[3]]);
        let crc16 = u16::from_be_bytes([b[4], b[5]]);
        let flags = u16::from_be_bytes([b[6], b[7]]);
        let payload_len = u16::from_be_bytes([b[8], b[9]]);
        let mut reserved = [0u8; 6];
        if b.len() >= 16 {
            reserved.copy_from_slice(&b[10..16]);
        }
        Self {
            page_id,
            crc16,
            flags,
            payload_len,
            reserved,
        }
    }
}

/// 544-byte physical page representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GdbPage {
    pub header: GdbPageHeader,
    pub payload: [u8; GDB_PAYLOAD_SIZE],
    pub trailer_sync: u32,
    pub trailer_padding: [u8; 12],
}

impl GdbPage {
    pub fn new(page_id: u32, flags: u16, payload_data: &[u8]) -> Self {
        let mut payload = [0u8; GDB_PAYLOAD_SIZE];
        let copy_len = payload_data.len().min(GDB_PAYLOAD_SIZE);
        payload[..copy_len].copy_from_slice(&payload_data[..copy_len]);

        let crc16 = gdb_crc16_ccitt(&payload);
        let header = GdbPageHeader {
            page_id,
            crc16,
            flags,
            payload_len: copy_len as u16,
            reserved: [0; 6],
        };

        Self {
            header,
            payload,
            trailer_sync: GDB_TRAILER_SYNC,
            trailer_padding: [0; 12],
        }
    }

    pub fn to_bytes(&self) -> [u8; GDB_PAGE_SIZE] {
        let mut page = [0u8; GDB_PAGE_SIZE];
        page[0..16].copy_from_slice(&self.header.serialize());
        page[16..528].copy_from_slice(&self.payload);
        page[528..532].copy_from_slice(&self.trailer_sync.to_be_bytes());
        page[532..544].copy_from_slice(&self.trailer_padding);
        page
    }

    pub fn parse(data: &[u8]) -> Result<Self, CoreError> {
        if data.len() < GDB_PAGE_SIZE {
            return Err(CoreError::ImmutabilityViolation(format!(
                "Page buffer too small: expected {}, got {}",
                GDB_PAGE_SIZE,
                data.len()
            )));
        }

        let sync = u32::from_be_bytes([data[528], data[529], data[530], data[531]]);
        if sync != GDB_TRAILER_SYNC {
            return Err(CoreError::ImmutabilityViolation(format!(
                "Invalid GDB trailer sync: expected 0x{:08X}, got 0x{:08X}",
                GDB_TRAILER_SYNC, sync
            )));
        }

        let mut payload = [0u8; GDB_PAYLOAD_SIZE];
        payload.copy_from_slice(&data[16..528]);
        let computed_crc = gdb_crc16_ccitt(&payload);

        let stored_crc_primary = u16::from_be_bytes([data[4], data[5]]);
        let stored_crc_alt = u16::from_be_bytes([data[8], data[9]]);
        let crc_matches = computed_crc == stored_crc_primary || computed_crc == stored_crc_alt;

        if !crc_matches {
            return Err(CoreError::ImmutabilityViolation(format!(
                "GDB Page CRC16 mismatch: stored primary 0x{:04X} / alt 0x{:04X}, computed 0x{:04X}",
                stored_crc_primary, stored_crc_alt, computed_crc
            )));
        }

        let header = if computed_crc == stored_crc_primary {
            GdbPageHeader::parse(&data[0..16])
        } else {
            let page_id = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
            let payload_len = u16::from_be_bytes([data[4], data[5]]);
            let flags = u16::from_be_bytes([data[6], data[7]]);
            let mut reserved = [0u8; 6];
            reserved.copy_from_slice(&data[10..16]);
            GdbPageHeader {
                page_id,
                crc16: stored_crc_alt,
                flags,
                payload_len,
                reserved,
            }
        };

        let mut trailer_padding = [0u8; 12];
        trailer_padding.copy_from_slice(&data[532..544]);

        Ok(Self {
            header,
            payload,
            trailer_sync: sync,
            trailer_padding,
        })
    }

    pub fn verify_crc(&self) -> bool {
        gdb_crc16_ccitt(&self.payload) == self.header.crc16
    }
}

/// Creates a physical 544-byte GDB page with 16-byte header, 512-byte payload, and sync guard
pub fn create_gdb_page(page_idx: u32, payload: &[u8]) -> Vec<u8> {
    GdbPage::new(page_idx, 0, payload).to_bytes().to_vec()
}

#[derive(Debug, Clone)]
pub struct HbGdb {
    pub header: GdbMasterHeader,
    pub total_pages: usize,
    pub total_size: usize,
}

impl HbGdb {
    pub fn parse(data: &[u8]) -> Result<Self, CoreError> {
        if data.len() < GDB_PAGE_SIZE {
            // Check legacy 12-byte stub compatibility
            if data.len() >= 12 && &data[0..4] == GDB_MAGIC {
                let version = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
                let flags = u32::from_be_bytes([data[8], data[9], data[10], data[11]]);
                return Ok(Self {
                    header: GdbMasterHeader {
                        magic: *GDB_MAGIC,
                        version,
                        flags,
                        total_pages: 1,
                        ..Default::default()
                    },
                    total_pages: 1,
                    total_size: data.len(),
                });
            }
            return Err(CoreError::ImmutabilityViolation(
                "GDB file too small for 544-byte page".into(),
            ));
        }

        // Verify page 0 trailer sync
        let sync = u32::from_be_bytes([data[528], data[529], data[530], data[531]]);
        if sync != GDB_TRAILER_SYNC {
            // Check if legacy unpaged format
            if &data[0..4] == GDB_MAGIC {
                let version = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
                let flags = u32::from_be_bytes([data[8], data[9], data[10], data[11]]);
                return Ok(Self {
                    header: GdbMasterHeader {
                        magic: *GDB_MAGIC,
                        version,
                        flags,
                        total_pages: (data.len() / GDB_PAGE_SIZE) as u32,
                        ..Default::default()
                    },
                    total_pages: 1,
                    total_size: data.len(),
                });
            }
            return Err(CoreError::ImmutabilityViolation(
                "Invalid GDB trailer sync marker".into(),
            ));
        }

        // Check CRC-16 of page 0 (supports primary offset 4..6 or legacy offset 8..10)
        let stored_crc_primary = u16::from_be_bytes([data[4], data[5]]);
        let stored_crc_alt = u16::from_be_bytes([data[8], data[9]]);
        let computed_crc = gdb_crc16_ccitt(&data[16..528]);
        if stored_crc_primary != computed_crc && stored_crc_alt != computed_crc {
            return Err(CoreError::ImmutabilityViolation(format!(
                "GDB Page 0 CRC16 mismatch: stored 0x{:04X}, computed 0x{:04X}",
                stored_crc_primary, computed_crc
            )));
        }

        let header = GdbMasterHeader::parse(&data[16..528])?;
        let total_pages = data.len() / GDB_PAGE_SIZE;

        Ok(Self {
            header,
            total_pages,
            total_size: data.len(),
        })
    }
}

/// High-level reader for Harman/Becker GDB v37 databases
#[derive(Debug, Clone)]
pub struct HbGdbReader {
    pub header: GdbMasterHeader,
    pub pages: Vec<GdbPage>,
    pub total_size: usize,
}

impl HbGdbReader {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, CoreError> {
        let bytes = std::fs::read(path)?;
        Self::from_bytes(&bytes)
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, CoreError> {
        let gdb = HbGdb::parse(data)?;
        let page_count = data.len() / GDB_PAGE_SIZE;
        let mut pages = Vec::with_capacity(page_count);

        if page_count > 0 && data.len() >= GDB_PAGE_SIZE {
            let sync = u32::from_be_bytes([data[528], data[529], data[530], data[531]]);
            if sync == GDB_TRAILER_SYNC {
                for i in 0..page_count {
                    let offset = i * GDB_PAGE_SIZE;
                    let page = GdbPage::parse(&data[offset..offset + GDB_PAGE_SIZE])?;
                    pages.push(page);
                }
            }
        }

        Ok(Self {
            header: gdb.header,
            pages,
            total_size: data.len(),
        })
    }

    pub fn header(&self) -> &GdbMasterHeader {
        &self.header
    }

    pub fn page_count(&self) -> usize {
        self.pages.len()
    }

    pub fn page(&self, page_id: u32) -> Option<&GdbPage> {
        self.pages.get(page_id as usize)
    }

    pub fn validate_all_pages(&self) -> Result<(), CoreError> {
        for page in &self.pages {
            if !page.verify_crc() {
                return Err(CoreError::ImmutabilityViolation(format!(
                    "CRC check failed on GDB page {}",
                    page.header.page_id
                )));
            }
            if page.trailer_sync != GDB_TRAILER_SYNC {
                return Err(CoreError::ImmutabilityViolation(format!(
                    "Trailer sync failed on GDB page {}",
                    page.header.page_id
                )));
            }
        }
        Ok(())
    }

    pub fn read_nodes(&self) -> Result<Vec<GdbNodeRecord>, CoreError> {
        let mut nodes = Vec::new();
        let first_node_page = 1usize;
        let mut last_node_page = self.pages.len();

        if self.header.morton_root_page > 1 && (self.header.morton_root_page as usize) < last_node_page {
            last_node_page = self.header.morton_root_page as usize;
        }
        for &off in &self.header.frc_layer_offsets {
            if off > 1 && (off as usize) < last_node_page {
                last_node_page = off as usize;
            }
        }

        for page_idx in first_node_page..last_node_page {
            if let Some(page) = self.pages.get(page_idx) {
                let valid_len = (page.header.payload_len as usize).min(GDB_PAYLOAD_SIZE);
                let record_count = valid_len / 16;
                for i in 0..record_count {
                    let off = i * 16;
                    nodes.push(GdbNodeRecord::parse(&page.payload[off..off + 16]));
                    if nodes.len() >= self.header.node_count as usize {
                        return Ok(nodes);
                    }
                }
            }
        }
        Ok(nodes)
    }

    pub fn read_edges_frc(&self, frc: u8) -> Result<Vec<GdbEdgeRecord>, CoreError> {
        let mut edges = Vec::new();
        let frc_idx = frc as usize;
        if frc_idx >= 8 {
            return Ok(edges);
        }
        let start_page = self.header.frc_layer_offsets[frc_idx] as usize;
        if start_page == 0 || start_page >= self.pages.len() {
            return Ok(edges);
        }

        let mut next_page = self.pages.len();
        for &offset in &self.header.frc_layer_offsets[frc_idx + 1..8] {
            if offset > start_page as u32 && (offset as usize) < next_page {
                next_page = offset as usize;
            }
        }

        let target_count = self.header.frc_layer_counts[frc_idx] as usize;

        for page_idx in start_page..next_page {
            if let Some(page) = self.pages.get(page_idx) {
                let valid_len = (page.header.payload_len as usize).min(GDB_PAYLOAD_SIZE);
                let record_count = valid_len / 24;
                for i in 0..record_count {
                    let off = i * 24;
                    let edge = GdbEdgeRecord::parse(&page.payload[off..off + 24]);
                    if edge.frc == frc {
                        edges.push(edge);
                        if target_count > 0 && edges.len() >= target_count {
                            return Ok(edges);
                        }
                    }
                }
            }
        }
        Ok(edges)
    }

    pub fn read_all_edges(&self) -> Result<Vec<GdbEdgeRecord>, CoreError> {
        let mut all_edges = Vec::new();
        for frc in 0..8 {
            let mut frc_edges = self.read_edges_frc(frc)?;
            all_edges.append(&mut frc_edges);
        }
        Ok(all_edges)
    }

    pub fn read_restrictions(&self) -> Result<Vec<GdbTurnRestrictionRecord>, CoreError> {
        let mut restrictions = Vec::new();
        if self.header.restriction_count == 0 {
            return Ok(restrictions);
        }

        let max_frc = self.header.frc_layer_offsets.iter().copied().max().unwrap_or(0) as usize;
        let start_page = if max_frc > 0 { max_frc + 1 } else { 1 };

        for page_idx in start_page..self.pages.len() {
            if let Some(page) = self.pages.get(page_idx) {
                let valid_len = (page.header.payload_len as usize).min(GDB_PAYLOAD_SIZE);
                let record_count = valid_len / 24;
                for i in 0..record_count {
                    let off = i * 24;
                    restrictions.push(GdbTurnRestrictionRecord::parse(&page.payload[off..off + 24]));
                    if restrictions.len() >= self.header.restriction_count as usize {
                        return Ok(restrictions);
                    }
                }
            }
        }
        Ok(restrictions)
    }

    pub fn read_spatial_tiles(&self) -> Result<Vec<GdbSpatialTileRecord>, CoreError> {
        let mut tiles = Vec::new();
        let root = self.header.morton_root_page as usize;
        if root == 0 || root >= self.pages.len() {
            return Ok(tiles);
        }

        let tile_count = self.header.morton_tile_count as usize;
        for page_idx in root..self.pages.len() {
            if let Some(page) = self.pages.get(page_idx) {
                let valid_len = (page.header.payload_len as usize).min(GDB_PAYLOAD_SIZE);
                let record_count = valid_len / 48;
                for i in 0..record_count {
                    let off = i * 48;
                    tiles.push(GdbSpatialTileRecord::parse(&page.payload[off..off + 48]));
                    if tile_count > 0 && tiles.len() >= tile_count {
                        return Ok(tiles);
                    }
                }
            }
        }
        Ok(tiles)
    }
}

/// Writer for Harman/Becker GDB v37 databases
pub struct HbGdbWriter<W: Write + Seek> {
    writer: W,
    pages_written: u32,
}

impl<W: Write + Seek> HbGdbWriter<W> {
    pub fn new(mut writer: W) -> Result<Self, CoreError> {
        let page0 = [0u8; GDB_PAGE_SIZE];
        writer.write_all(&page0)?;
        Ok(Self {
            writer,
            pages_written: 1,
        })
    }

    pub fn write_page(&mut self, page: &GdbPage) -> Result<u32, CoreError> {
        let page_id = self.pages_written;
        let mut p = page.clone();
        p.header.page_id = page_id;
        p.header.crc16 = gdb_crc16_ccitt(&p.payload);
        self.writer.write_all(&p.to_bytes())?;
        self.pages_written += 1;
        Ok(page_id)
    }

    pub fn write_payload_page(&mut self, flags: u16, payload: &[u8]) -> Result<u32, CoreError> {
        let page_id = self.pages_written;
        let page = GdbPage::new(page_id, flags, payload);
        self.writer.write_all(&page.to_bytes())?;
        self.pages_written += 1;
        Ok(page_id)
    }

    pub fn finalize(&mut self, header: &GdbMasterHeader) -> Result<(), CoreError> {
        let mut h = header.clone();
        h.total_pages = self.pages_written;
        let page0 = GdbPage::new(0, 0, &h.serialize_payload());
        self.writer.seek(SeekFrom::Start(0))?;
        self.writer.write_all(&page0.to_bytes())?;
        self.writer.flush()?;
        Ok(())
    }

    pub fn pages_written(&self) -> u32 {
        self.pages_written
    }
}

#[derive(Debug, Clone)]
pub struct HbGdbAdapter {
    capabilities: FormatCapabilities,
}

impl Default for HbGdbAdapter {
    fn default() -> Self {
        Self {
            capabilities: FormatCapabilities::read_only(),
        }
    }
}

impl FormatAdapter for HbGdbAdapter {
    fn format_name(&self) -> &'static str {
        "hb_gdb"
    }

    fn capabilities(&self) -> &FormatCapabilities {
        &self.capabilities
    }

    fn detect(&self, data: &[u8]) -> bool {
        if data.len() < 4 {
            return false;
        }
        // Either matches at byte 0 or inside Page 0 payload offset 16
        &data[0..4] == GDB_MAGIC || (data.len() >= 20 && &data[16..20] == GDB_MAGIC)
    }

    fn coverage_ratio(&self, data: &[u8]) -> f32 {
        if self.detect(data) {
            1.0
        } else {
            0.0
        }
    }
}
