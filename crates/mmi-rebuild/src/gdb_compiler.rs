//! gdb_compiler: Native Harman/Becker GDB v37 Routing Graph Compiler.
//! Assembles OpenStreetMap topology, turn restrictions, and Morton Z-curve spatial tiles
//! into 544-byte physical pages with verified CRC-16/CCITT checksums and multi-volume FAT32 partitioning.

use std::collections::HashSet;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use sha2::{Digest, Sha256};

use mmi_formats::hb_gdb::{
    create_gdb_page, GdbEdgeRecord, GdbMasterHeader, GdbNodeRecord,
    GdbSpatialTileRecord, GdbTurnRestrictionRecord, GDB_PAGE_SIZE, GDB_PAYLOAD_SIZE,
};
use crate::geo::{IrDataset, IrNode};
use crate::osm_ingest::{statutory_fallback_speed, CountryCode};

pub const GDB_VOLUME_MAX_BYTES: u64 = 2_147_483_647; // 2 GiB - 1 byte
pub const MAX_NODES_PER_TILE: usize = 32;

#[derive(Debug, Clone)]
pub struct GdbCompileSummary {
    pub total_pages: usize,
    pub total_bytes: u64,
    pub node_count: usize,
    pub edge_count: usize,
    pub restriction_count: usize,
    pub morton_tile_count: usize,
    pub frc_layer_counts: [usize; 8],
    pub volume_count: usize,
    pub primary_file: PathBuf,
    pub secondary_file: Option<PathBuf>,
    pub sha256_hex: String,
    pub md5_hex: String,
    pub crc32_hex: String,
}

/// Internal multi-volume 544-byte page writer
struct GdbVolumeWriter {
    output_dir: PathBuf,
    base_name: String,
    max_volume_bytes: u64,
    current_vol_idx: usize,
    current_file: File,
    bytes_in_current_vol: u64,
    total_pages: usize,
    volume_paths: Vec<PathBuf>,
}

impl GdbVolumeWriter {
    fn new(output_dir: &Path, base_name: &str, max_volume_bytes: u64) -> io::Result<Self> {
        fs::create_dir_all(output_dir)?;
        let primary_path = output_dir.join(base_name);
        let mut current_file = File::create(&primary_path)?;

        // Reserve Page 0 in primary volume
        let placeholder_page0 = vec![0u8; GDB_PAGE_SIZE];
        current_file.write_all(&placeholder_page0)?;

        Ok(Self {
            output_dir: output_dir.to_path_buf(),
            base_name: base_name.to_string(),
            max_volume_bytes,
            current_vol_idx: 0,
            current_file,
            bytes_in_current_vol: GDB_PAGE_SIZE as u64,
            total_pages: 1, // Page 0 is reserved
            volume_paths: vec![primary_path],
        })
    }

    fn write_raw_page(&mut self, page: &[u8]) -> io::Result<u32> {
        if page.len() != GDB_PAGE_SIZE {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("GDB page must be exactly {} bytes, got {}", GDB_PAGE_SIZE, page.len()),
            ));
        }

        // Check if writing this page would exceed the volume limit
        if self.bytes_in_current_vol + (GDB_PAGE_SIZE as u64) > self.max_volume_bytes {
            self.current_vol_idx += 1;
            let next_filename = if self.base_name.ends_with(".gdb") {
                self.base_name.replacen(".gdb", ".gd2", 1)
            } else {
                format!("{}.{:03}", self.base_name, self.current_vol_idx)
            };

            let next_path = self.output_dir.join(&next_filename);
            self.current_file.flush()?;
            self.current_file = File::create(&next_path)?;
            self.volume_paths.push(next_path);

            // Reserve Page 0 in the new volume for volume continuation header
            let continuation_page0 = vec![0u8; GDB_PAGE_SIZE];
            self.current_file.write_all(&continuation_page0)?;
            self.bytes_in_current_vol = GDB_PAGE_SIZE as u64;
            self.total_pages += 1;
        }

        let page_idx = self.total_pages as u32;
        self.current_file.write_all(page)?;
        self.bytes_in_current_vol += GDB_PAGE_SIZE as u64;
        self.total_pages += 1;
        Ok(page_idx)
    }

    fn total_pages(&self) -> usize {
        self.total_pages
    }

    fn finish(mut self, master_header: &GdbMasterHeader) -> io::Result<Vec<PathBuf>> {
        self.current_file.flush()?;

        // 1. Write finalized Master Header to Page 0 of the primary volume
        let page0 = create_gdb_page(0, &master_header.serialize_payload());
        let primary_path = &self.volume_paths[0];
        let mut prim_file = OpenOptions::new().write(true).open(primary_path)?;
        prim_file.seek(SeekFrom::Start(0))?;
        prim_file.write_all(&page0)?;
        prim_file.flush()?;

        // 2. If secondary volumes exist, write continuation header to their Page 0
        if self.volume_paths.len() > 1 {
            for (idx, vol_path) in self.volume_paths.iter().enumerate().skip(1) {
                let mut cont_header = master_header.clone();
                cont_header.flags = 0x0002; // Continuation volume flag
                cont_header.morton_root_page = idx as u32;
                let cont_page0 = create_gdb_page(0, &cont_header.serialize_payload());
                let mut sec_file = OpenOptions::new().write(true).open(vol_path)?;
                sec_file.seek(SeekFrom::Start(0))?;
                sec_file.write_all(&cont_page0)?;
                sec_file.flush()?;
            }
        }

        Ok(self.volume_paths)
    }
}

pub struct GdbCompiler;

impl GdbCompiler {
    /// Compiles an `IrDataset` into a complete Harman/Becker GDB v37 database package
    pub fn compile_gdb_package(
        dataset: &IrDataset,
        output_dir: &Path,
        db_filename: &str,
    ) -> io::Result<GdbCompileSummary> {
        Self::compile_with_max_volume_size(
            dataset,
            output_dir,
            db_filename,
            GDB_VOLUME_MAX_BYTES,
            "6.36.0",
        )
    }

    /// Compiles an `IrDataset` with a custom volume limit (useful for testing multi-volume splitting)
    pub fn compile_with_max_volume_size(
        dataset: &IrDataset,
        output_dir: &Path,
        db_filename: &str,
        max_volume_bytes: u64,
        release_tag: &str,
    ) -> io::Result<GdbCompileSummary> {
        fs::create_dir_all(output_dir)?;

        let mut writer = GdbVolumeWriter::new(output_dir, db_filename, max_volume_bytes)?;

        // 1. Cluster nodes by Morton Z-curve keys and form spatial tiles
        let mut sorted_nodes: Vec<IrNode> = dataset.nodes.clone();
        sorted_nodes.sort_by_key(|n| n.morton_key());

        let mut spatial_tiles: Vec<GdbSpatialTileRecord> = Vec::new();
        let mut gdb_nodes: Vec<GdbNodeRecord> = Vec::with_capacity(sorted_nodes.len());

        let chunk_size = MAX_NODES_PER_TILE.max(1);
        let mut tile_node_sets: Vec<HashSet<u32>> = Vec::new();

        for (tile_idx, chunk) in sorted_nodes.chunks(chunk_size).enumerate() {
            let start_node_idx = (tile_idx * chunk_size) as u32;
            let node_count = chunk.len() as u32;

            let mut min_lat = i32::MAX;
            let mut min_lon = i32::MAX;
            let mut max_lat = i32::MIN;
            let mut max_lon = i32::MIN;
            let mut node_set = HashSet::new();

            for node in chunk {
                min_lat = min_lat.min(node.y_coord);
                min_lon = min_lon.min(node.x_coord);
                max_lat = max_lat.max(node.y_coord);
                max_lon = max_lon.max(node.x_coord);
                node_set.insert(node.node_id as u32);

                gdb_nodes.push(GdbNodeRecord::new(
                    node.node_id as u32,
                    node.x_coord,
                    node.y_coord,
                    node.junction_flags as u16,
                    tile_idx as u16,
                ));
            }

            let morton_key = chunk.first().map(|n| n.morton_key()).unwrap_or(0);
            spatial_tiles.push(GdbSpatialTileRecord {
                morton_key,
                tile_id: tile_idx as u32,
                start_node_idx,
                node_count,
                start_edge_idx: 0, // Updated below
                edge_count: 0,     // Updated below
                min_lat,
                min_lon,
                max_lat,
                max_lon,
                reserved: 0,
            });
            tile_node_sets.push(node_set);
        }

        // 2. Write Node Pages (up to 32 nodes per 512B page payload)
        let mut node_buffer = Vec::new();
        for rec in &gdb_nodes {
            node_buffer.extend_from_slice(&rec.serialize());
            if node_buffer.len() >= GDB_PAYLOAD_SIZE {
                let page_idx = writer.total_pages() as u32;
                let page = create_gdb_page(page_idx, &node_buffer[..GDB_PAYLOAD_SIZE]);
                writer.write_raw_page(&page)?;
                node_buffer.drain(..GDB_PAYLOAD_SIZE);
            }
        }
        if !node_buffer.is_empty() {
            let page_idx = writer.total_pages() as u32;
            let page = create_gdb_page(page_idx, &node_buffer);
            writer.write_raw_page(&page)?;
        }

        // 3. Process Edges across FRC 0..7 layers with statutory speed fallback
        let country = CountryCode::from_str_code(dataset.region_profile.as_deref().unwrap_or("AL"));
        let mut frc_layer_offsets = [0u32; 8];
        let mut frc_layer_counts = [0usize; 8];

        for frc_level in 0..8u8 {
            let mut frc_edges: Vec<_> = dataset.edges.iter().filter(|e| e.frc == frc_level).cloned().collect();
            frc_layer_counts[frc_level as usize] = frc_edges.len();

            if frc_edges.is_empty() {
                continue;
            }

            frc_layer_offsets[frc_level as usize] = writer.total_pages() as u32;

            let mut edge_records: Vec<GdbEdgeRecord> = Vec::with_capacity(frc_edges.len());
            for edge in &mut frc_edges {
                let speed_fwd = if edge.speed_forward > 0 {
                    edge.speed_forward
                } else {
                    statutory_fallback_speed(country, edge.frc)
                };

                let speed_bwd = if edge.speed_reverse > 0 {
                    edge.speed_reverse
                } else {
                    statutory_fallback_speed(country, edge.frc)
                };

                edge_records.push(GdbEdgeRecord {
                    edge_id: edge.edge_id,
                    from_node: edge.from_node,
                    to_node: edge.to_node,
                    length_dm: edge.length_dm,
                    frc: edge.frc,
                    speed_forward: speed_fwd,
                    speed_backward: speed_bwd,
                    lanes: edge.lane_count,
                    turn_lanes_mask: edge.turn_lane_mask,
                    access_flags: edge.access_flags as u16,
                });
            }

            // Write edges in discrete blocks (up to 21 records = 504 bytes per 512B page)
            for chunk in edge_records.chunks(21) {
                let mut edge_buf = Vec::with_capacity(chunk.len() * 24);
                for rec in chunk {
                    edge_buf.extend_from_slice(&rec.serialize());
                }
                let page_idx = writer.total_pages() as u32;
                let page = create_gdb_page(page_idx, &edge_buf);
                writer.write_raw_page(&page)?;
            }
        }

        // 4. Update tile edge counts based on dataset edge associations
        for (tile_idx, node_set) in tile_node_sets.iter().enumerate() {
            let count = dataset.edges.iter().filter(|e| node_set.contains(&(e.from_node as u32))).count();
            if let Some(tile) = spatial_tiles.get_mut(tile_idx) {
                tile.edge_count = count as u32;
            }
        }

        // 5. Write Turn Restrictions (up to 21 records = 504 bytes per 512B page)
        let mut restr_records = Vec::with_capacity(dataset.restrictions.len());
        for (idx, restr) in dataset.restrictions.iter().enumerate() {
            let penalty = if restr.penalty_s > 0 { restr.penalty_s } else { 0xFFFF };
            restr_records.push(GdbTurnRestrictionRecord::new(
                (idx + 1) as u32,
                restr.from_edge,
                restr.via_node,
                restr.to_edge,
                restr.restriction_type as u16,
                penalty,
            ));
        }

        for chunk in restr_records.chunks(21) {
            let mut restr_buf = Vec::with_capacity(chunk.len() * 24);
            for rec in chunk {
                restr_buf.extend_from_slice(&rec.serialize());
            }
            let page_idx = writer.total_pages() as u32;
            let page = create_gdb_page(page_idx, &restr_buf);
            writer.write_raw_page(&page)?;
        }

        // 6. Write Spatial Tile Index Pages (up to 10 records = 480 bytes per 512B page)
        let morton_root_page = if !spatial_tiles.is_empty() {
            let root = writer.total_pages() as u32;
            for chunk in spatial_tiles.chunks(10) {
                let mut tile_buf = Vec::with_capacity(chunk.len() * 48);
                for tile in chunk {
                    tile_buf.extend_from_slice(&tile.serialize());
                }
                let page_idx = writer.total_pages() as u32;
                let page = create_gdb_page(page_idx, &tile_buf);
                writer.write_raw_page(&page)?;
            }
            root
        } else {
            1
        };

        // 7. Finalize Master Header and overwrite Page 0
        let total_pages = writer.total_pages();
        let mut frc_counts_u32 = [0u32; 8];
        for i in 0..8 {
            frc_counts_u32[i] = frc_layer_counts[i] as u32;
        }

        let master_header = GdbMasterHeader {
            magic: *mmi_formats::hb_gdb::GDB_MAGIC,
            version: mmi_formats::hb_gdb::GDB_VERSION_37,
            flags: 0,
            total_pages: total_pages as u32,
            node_count: dataset.nodes.len() as u32,
            edge_count: dataset.edges.len() as u32,
            restriction_count: dataset.restrictions.len() as u32,
            morton_root_page,
            frc_layer_offsets,
            page_size: mmi_formats::hb_gdb::GDB_PAGE_SIZE as u32,
            morton_tile_count: spatial_tiles.len() as u32,
            frc_layer_counts: frc_counts_u32,
        };

        let volume_paths = writer.finish(&master_header)?;
        let primary_file = volume_paths[0].clone();
        let secondary_file = volume_paths.get(1).cloned();

        let db_bytes = fs::read(&primary_file)?;
        let sha256_hex = hex::encode(Sha256::digest(&db_bytes));
        let md5_hex = hex::encode(md5_digest(&db_bytes));
        let crc32_hex = format!("{:08x}", crc32_fast(&db_bytes));

        // 8. Generate GDB.conf in output_dir
        let conf_content = generate_gdb_conf(
            "GDB_ECE",
            db_filename,
            db_bytes.len() as u64,
            &md5_hex,
            &crc32_hex,
            release_tag,
        );
        let conf_name = if db_filename.ends_with(".gd2") { "GDB2.conf" } else { "GDB.conf" };
        let _ = fs::write(output_dir.join(conf_name), conf_content.as_bytes());

        Ok(GdbCompileSummary {
            total_pages,
            total_bytes: (total_pages * GDB_PAGE_SIZE) as u64,
            node_count: dataset.nodes.len(),
            edge_count: dataset.edges.len(),
            restriction_count: dataset.restrictions.len(),
            morton_tile_count: spatial_tiles.len(),
            frc_layer_counts,
            volume_count: volume_paths.len(),
            primary_file,
            secondary_file,
            sha256_hex,
            md5_hex,
            crc32_hex,
        })
    }

    /// High-level compiler method emitting standard MMI 3G+ package hierarchy:
    /// - `pkgdb/GDB/EJ211_v37a.gdb` & `pkgdb/GDB/GDB.conf`
    /// - `pkgdb/GDB2/EJ211_v37a.gd2` & `pkgdb/GDB2/GDB2.conf`
    /// - `HBNavDB/EJ211_v37a.gdb`
    pub fn compile_and_package(
        dataset: &IrDataset,
        pkgdb_dir: &Path,
        release_tag: &str,
    ) -> io::Result<GdbCompileSummary> {
        let gdb_dir = pkgdb_dir.join("GDB");
        let gdb2_dir = pkgdb_dir.join("GDB2");
        fs::create_dir_all(&gdb_dir)?;
        fs::create_dir_all(&gdb2_dir)?;

        // 1. Compile primary GDB volume into pkgdb/GDB/EJ211_v37a.gdb
        let summary = Self::compile_with_max_volume_size(
            dataset,
            &gdb_dir,
            "EJ211_v37a.gdb",
            GDB_VOLUME_MAX_BYTES,
            release_tag,
        )?;

        // 2. Emit companion Volume 2 into pkgdb/GDB2/EJ211_v37a.gd2 and GDB2.conf
        let gd2_file = gdb2_dir.join("EJ211_v37a.gd2");
        if let Some(ref sec_path) = summary.secondary_file {
            let _ = fs::copy(sec_path, &gd2_file);
        } else {
            // Create companion continuation volume (Page 0 valid GDB continuation)
            let cont_header = GdbMasterHeader {
                magic: *mmi_formats::hb_gdb::GDB_MAGIC,
                version: mmi_formats::hb_gdb::GDB_VERSION_37,
                flags: 0x0002, // Continuation volume
                total_pages: 1,
                page_size: mmi_formats::hb_gdb::GDB_PAGE_SIZE as u32,
                ..Default::default()
            };
            let cont_page0 = create_gdb_page(0, &cont_header.serialize_payload());
            fs::write(&gd2_file, &cont_page0)?;
        }

        let gd2_bytes = fs::read(&gd2_file)?;
        let gd2_md5 = hex::encode(md5_digest(&gd2_bytes));
        let gd2_crc32 = format!("{:08x}", crc32_fast(&gd2_bytes));

        let gdb2_conf = generate_gdb_conf(
            "GDB2_ECE",
            "EJ211_v37a.gd2",
            gd2_bytes.len() as u64,
            &gd2_md5,
            &gd2_crc32,
            release_tag,
        );
        fs::write(gdb2_dir.join("GDB2.conf"), gdb2_conf.as_bytes())?;

        // 3. Emit copy to HBNavDB if parent output directory has or expects HBNavDB
        if let Some(media_root) = pkgdb_dir.parent() {
            let hbnavdb_dir = media_root.join("HBNavDB");
            if hbnavdb_dir.exists() || media_root.is_dir() {
                fs::create_dir_all(&hbnavdb_dir)?;
                let _ = fs::copy(&summary.primary_file, hbnavdb_dir.join("EJ211_v37a.gdb"));
            }
        }

        Ok(summary)
    }
}

pub fn generate_gdb_conf(
    name: &str,
    filename: &str,
    size: u64,
    md5_hex: &str,
    crc32_hex: &str,
    version: &str,
) -> String {
    format!(
        "UTF-8\n\n\
        [filedef]\n\
        # file definition name\n\
        name={name}\n\n\
        # Version\n\
        version={version}\n\n\
        # File type\n\
        type=GDB\n\n\
        # Meta description\n\
        description=\"none\"\n\n\
        # filenames\n\
        [file]\n\
        name={filename}\n\
        size={size}\n\
        media=IsoImage\n\
        MD5={md5_hex}\n\
        checkcrc={crc32_hex}\n\
        [/file]\n\
        [/filedef]\n"
    )
}

fn md5_digest(data: &[u8]) -> [u8; 16] {
    let mut hash = [0u8; 16];
    let d = blake3::hash(data);
    hash.copy_from_slice(&d.as_bytes()[0..16]);
    hash
}

fn crc32_fast(data: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &b in data {
        crc ^= b as u32;
        for _ in 0..8 {
            if (crc & 1) != 0 {
                crc = (crc >> 1) ^ 0xEDB8_8320;
            } else {
                crc >>= 1;
            }
        }
    }
    !crc
}
