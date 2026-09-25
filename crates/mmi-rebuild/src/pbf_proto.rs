//! pbf_proto: Zero-copy Protobuf wire decoder and streaming OSM PBF block unpacker.
//!
//! Conforms to Protocol Buffers wire encoding specification and
//! OpenStreetMap PBF (ProtoBinaryFormat) fileblock layouts.

use std::collections::HashMap;
use std::io::{self, Read};
use flate2::read::ZlibDecoder;
use crate::osm_ingest::{OsmIngestError, RawOsmNode, RawOsmRelation, RawOsmWay};

/// Wire types in Protocol Buffers binary encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtoWireType {
    Varint = 0,
    Fixed64 = 1,
    LengthDelimited = 2,
    Fixed32 = 5,
}

impl ProtoWireType {
    pub fn from_u32(val: u32) -> Result<Self, OsmIngestError> {
        match val {
            0 => Ok(Self::Varint),
            1 => Ok(Self::Fixed64),
            2 => Ok(Self::LengthDelimited),
            5 => Ok(Self::Fixed32),
            other => Err(OsmIngestError::PbfDecodeError(format!("Invalid wire type: {}", other))),
        }
    }
}

/// Zero-allocation Protobuf wire reader parsing fields from an in-memory byte slice.
pub struct ProtoWireReader<'a> {
    buf: &'a [u8],
    cursor: usize,
}

impl<'a> ProtoWireReader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, cursor: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.cursor >= self.buf.len()
    }

    pub fn remaining(&self) -> usize {
        self.buf.len().saturating_sub(self.cursor)
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Reads next field tag, returning `(field_number, wire_type)`.
    pub fn next_tag(&mut self) -> Result<Option<(u32, ProtoWireType)>, OsmIngestError> {
        if self.is_empty() {
            return Ok(None);
        }
        let tag = self.read_varint()?;
        let field_num = (tag >> 3) as u32;
        let wire_type = ProtoWireType::from_u32((tag & 0x07) as u32)?;
        Ok(Some((field_num, wire_type)))
    }

    /// Decodes an unsigned 64-bit base-128 varint.
    pub fn read_varint(&mut self) -> Result<u64, OsmIngestError> {
        let mut result: u64 = 0;
        let mut shift = 0;

        while self.cursor < self.buf.len() {
            let byte = self.buf[self.cursor];
            self.cursor += 1;

            result |= ((byte & 0x7F) as u64) << shift;
            if (byte & 0x80) == 0 {
                return Ok(result);
            }
            shift += 7;
            if shift >= 64 {
                return Err(OsmIngestError::PbfDecodeError("Varint overflow (>64 bits)".to_string()));
            }
        }
        Err(OsmIngestError::PbfDecodeError("Unexpected EOF while reading varint".to_string()))
    }

    /// Decodes a signed 64-bit ZigZag-encoded varint (sint64).
    #[inline]
    pub fn read_sint64(&mut self) -> Result<i64, OsmIngestError> {
        let n = self.read_varint()?;
        Ok(((n >> 1) as i64) ^ (-((n & 1) as i64)))
    }

    /// Decodes a signed 32-bit ZigZag-encoded varint (sint32).
    #[inline]
    pub fn read_sint32(&mut self) -> Result<i32, OsmIngestError> {
        let n = self.read_varint()? as u32;
        Ok(((n >> 1) as i32) ^ (-((n & 1) as i32)))
    }

    /// Decodes an unsigned 32-bit varint.
    #[inline]
    pub fn read_uint32(&mut self) -> Result<u32, OsmIngestError> {
        self.read_varint().map(|v| v as u32)
    }

    /// Decodes a signed 32-bit standard varint (int32).
    #[inline]
    pub fn read_int32(&mut self) -> Result<i32, OsmIngestError> {
        self.read_varint().map(|v| v as i32)
    }

    /// Reads length-delimited byte slice.
    pub fn read_bytes(&mut self) -> Result<&'a [u8], OsmIngestError> {
        let len = self.read_varint()? as usize;
        if self.cursor + len > self.buf.len() {
            return Err(OsmIngestError::PbfDecodeError("Length-delimited slice out of bounds".to_string()));
        }
        let slice = &self.buf[self.cursor..self.cursor + len];
        self.cursor += len;
        Ok(slice)
    }

    /// Reads length-delimited UTF-8 string slice.
    pub fn read_string(&mut self) -> Result<&'a str, OsmIngestError> {
        let bytes = self.read_bytes()?;
        std::str::from_utf8(bytes)
            .map_err(|e| OsmIngestError::PbfDecodeError(format!("Invalid UTF-8 string in protobuf: {}", e)))
    }

    /// Reads packed sint64 array from length-delimited bytes.
    pub fn read_packed_sint64(&mut self) -> Result<Vec<i64>, OsmIngestError> {
        let bytes = self.read_bytes()?;
        let mut reader = ProtoWireReader::new(bytes);
        let mut values = Vec::new();
        while !reader.is_empty() {
            values.push(reader.read_sint64()?);
        }
        Ok(values)
    }

    /// Reads packed uint32 array from length-delimited bytes.
    pub fn read_packed_uint32(&mut self) -> Result<Vec<u32>, OsmIngestError> {
        let bytes = self.read_bytes()?;
        let mut reader = ProtoWireReader::new(bytes);
        let mut values = Vec::new();
        while !reader.is_empty() {
            values.push(reader.read_uint32()?);
        }
        Ok(values)
    }

    /// Reads packed int32 array from length-delimited bytes.
    pub fn read_packed_int32(&mut self) -> Result<Vec<i32>, OsmIngestError> {
        let bytes = self.read_bytes()?;
        let mut reader = ProtoWireReader::new(bytes);
        let mut values = Vec::new();
        while !reader.is_empty() {
            values.push(reader.read_int32()?);
        }
        Ok(values)
    }

    /// Skips a field based on wire type.
    pub fn skip_field(&mut self, wire_type: ProtoWireType) -> Result<(), OsmIngestError> {
        match wire_type {
            ProtoWireType::Varint => {
                let _ = self.read_varint()?;
            }
            ProtoWireType::Fixed64 => {
                if self.cursor + 8 > self.buf.len() {
                    return Err(OsmIngestError::PbfDecodeError("Fixed64 out of bounds".to_string()));
                }
                self.cursor += 8;
            }
            ProtoWireType::LengthDelimited => {
                let _ = self.read_bytes()?;
            }
            ProtoWireType::Fixed32 => {
                if self.cursor + 4 > self.buf.len() {
                    return Err(OsmIngestError::PbfDecodeError("Fixed32 out of bounds".to_string()));
                }
                self.cursor += 4;
            }
        }
        Ok(())
    }
}

/// Metadata header describing an OSM PBF file block.
#[derive(Debug, Clone)]
pub struct PbfBlockHeader {
    pub block_type: String,
    pub datasize: usize,
}

/// Stream reader for unpacking PBF BlobHeader and Blob payloads.
pub struct PbfBlobReader;

impl PbfBlobReader {
    /// Reads 4-byte BE length and decodes `BlobHeader`.
    pub fn read_block_header<R: Read>(reader: &mut R) -> Result<Option<PbfBlockHeader>, OsmIngestError> {
        let mut len_buf = [0u8; 4];
        match reader.read_exact(&mut len_buf) {
            Ok(()) => {}
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => return Ok(None),
            Err(e) => return Err(OsmIngestError::IoError(e)),
        }

        let header_len = u32::from_be_bytes(len_buf) as usize;
        if header_len > 64 * 1024 {
            return Err(OsmIngestError::PbfDecodeError(format!(
                "BlobHeader exceeds 64KB limit: {} bytes",
                header_len
            )));
        }
        if header_len == 0 {
            return Err(OsmIngestError::PbfDecodeError("BlobHeader length is 0".to_string()));
        }

        let mut header_bytes = vec![0u8; header_len];
        reader.read_exact(&mut header_bytes).map_err(OsmIngestError::IoError)?;

        let mut wire = ProtoWireReader::new(&header_bytes);
        let mut block_type = String::new();
        let mut datasize: usize = 0;

        while let Some((field, wire_type)) = wire.next_tag()? {
            match (field, wire_type) {
                (1, ProtoWireType::LengthDelimited) => {
                    block_type = wire.read_string()?.to_string();
                }
                (3, ProtoWireType::Varint) => {
                    datasize = wire.read_varint()? as usize;
                }
                (_, wt) => wire.skip_field(wt)?,
            }
        }

        Ok(Some(PbfBlockHeader { block_type, datasize }))
    }

    /// Reads Blob and decompresses into `out_decompressed` buffer.
    pub fn read_decompressed_payload<R: Read>(
        reader: &mut R,
        datasize: usize,
        out_decompressed: &mut Vec<u8>,
    ) -> Result<(), OsmIngestError> {
        let mut blob_bytes = vec![0u8; datasize];
        reader.read_exact(&mut blob_bytes).map_err(OsmIngestError::IoError)?;

        let mut proto = ProtoWireReader::new(&blob_bytes);
        out_decompressed.clear();
        let mut found_payload = false;

        while let Some((field, wire_type)) = proto.next_tag()? {
            match (field, wire_type) {
                (1, ProtoWireType::LengthDelimited) => {
                    // Raw uncompressed bytes
                    let raw = proto.read_bytes()?;
                    out_decompressed.extend_from_slice(raw);
                    found_payload = true;
                    break;
                }
                (3, ProtoWireType::LengthDelimited) => {
                    // Zlib compressed stream
                    let zlib_data = proto.read_bytes()?;
                    let mut decoder = ZlibDecoder::new(zlib_data);
                    decoder
                        .read_to_end(out_decompressed)
                        .map_err(|e| OsmIngestError::PbfDecodeError(format!("Zlib decompress failed: {}", e)))?;
                    found_payload = true;
                    break;
                }
                (_, wt) => proto.skip_field(wt)?,
            }
        }

        if !found_payload {
            // Fallback: check if the blob payload itself has a raw zlib header (heuristic scan)
            for i in 0..blob_bytes.len().saturating_sub(2) {
                if blob_bytes[i] == 0x78 && (blob_bytes[i + 1] == 0x9C || blob_bytes[i + 1] == 0x01 || blob_bytes[i + 1] == 0xDA) {
                    let mut decoder = ZlibDecoder::new(&blob_bytes[i..]);
                    if decoder.read_to_end(out_decompressed).is_ok() && !out_decompressed.is_empty() {
                        return Ok(());
                    }
                }
            }
            return Err(OsmIngestError::PbfDecodeError("No valid raw or zlib payload found in Blob".to_string()));
        }

        Ok(())
    }
}

/// Parses an uncompressed PrimitiveBlock message.
pub struct PrimitiveBlockParser<'a> {
    data: &'a [u8],
}

impl<'a> PrimitiveBlockParser<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    /// Parses string table and primitive groups from a PrimitiveBlock.
    pub fn parse_block(
        &self,
        out_nodes: &mut Vec<RawOsmNode>,
        out_ways: &mut Vec<RawOsmWay>,
        out_relations: &mut Vec<RawOsmRelation>,
    ) -> Result<(), OsmIngestError> {
        let mut wire = ProtoWireReader::new(self.data);
        let mut string_table: Vec<String> = Vec::new();
        let mut primitive_groups: Vec<&'a [u8]> = Vec::new();
        let mut granularity: i64 = 100;
        let mut lat_offset: i64 = 0;
        let mut lon_offset: i64 = 0;

        while let Some((field, wire_type)) = wire.next_tag()? {
            match (field, wire_type) {
                (1, ProtoWireType::LengthDelimited) => {
                    // StringTable
                    let st_bytes = wire.read_bytes()?;
                    string_table = Self::parse_string_table(st_bytes)?;
                }
                (2, ProtoWireType::LengthDelimited) => {
                    // PrimitiveGroup
                    let pg_bytes = wire.read_bytes()?;
                    primitive_groups.push(pg_bytes);
                }
                (17, ProtoWireType::Varint) => {
                    granularity = wire.read_int32()? as i64;
                }
                (19, ProtoWireType::Varint) => {
                    lat_offset = wire.read_sint64()?;
                }
                (20, ProtoWireType::Varint) => {
                    lon_offset = wire.read_sint64()?;
                }
                (_, wt) => wire.skip_field(wt)?,
            }
        }

        let str_refs: Vec<&str> = string_table.iter().map(|s| s.as_str()).collect();

        for pg_bytes in primitive_groups {
            Self::parse_primitive_group(
                pg_bytes,
                &str_refs,
                granularity,
                lat_offset,
                lon_offset,
                out_nodes,
                out_ways,
                out_relations,
            )?;
        }

        Ok(())
    }

    fn parse_string_table(data: &[u8]) -> Result<Vec<String>, OsmIngestError> {
        let mut wire = ProtoWireReader::new(data);
        let mut strings = Vec::new();

        while let Some((field, wire_type)) = wire.next_tag()? {
            match (field, wire_type) {
                (1, ProtoWireType::LengthDelimited) => {
                    let bytes = wire.read_bytes()?;
                    let s = String::from_utf8_lossy(bytes).into_owned();
                    strings.push(s);
                }
                (_, wt) => wire.skip_field(wt)?,
            }
        }

        Ok(strings)
    }

    fn parse_primitive_group(
        data: &'a [u8],
        strings: &[&str],
        granularity: i64,
        lat_offset: i64,
        lon_offset: i64,
        out_nodes: &mut Vec<RawOsmNode>,
        out_ways: &mut Vec<RawOsmWay>,
        out_relations: &mut Vec<RawOsmRelation>,
    ) -> Result<(), OsmIngestError> {
        let mut wire = ProtoWireReader::new(data);

        while let Some((field, wire_type)) = wire.next_tag()? {
            match (field, wire_type) {
                (1, ProtoWireType::LengthDelimited) => {
                    // Standard Node (individual)
                    let node_bytes = wire.read_bytes()?;
                    if let Some(node) = Self::parse_single_node(node_bytes, strings, granularity, lat_offset, lon_offset)? {
                        out_nodes.push(node);
                    }
                }
                (2, ProtoWireType::LengthDelimited) => {
                    // DenseNodes
                    let dense_bytes = wire.read_bytes()?;
                    Self::parse_dense_nodes(dense_bytes, strings, granularity, lat_offset, lon_offset, out_nodes)?;
                }
                (3, ProtoWireType::LengthDelimited) => {
                    // Way
                    let way_bytes = wire.read_bytes()?;
                    if let Some(way) = Self::parse_way(way_bytes, strings)? {
                        out_ways.push(way);
                    }
                }
                (4, ProtoWireType::LengthDelimited) => {
                    // Relation
                    let rel_bytes = wire.read_bytes()?;
                    if let Some(rel) = Self::parse_relation(rel_bytes, strings)? {
                        out_relations.push(rel);
                    }
                }
                (_, wt) => wire.skip_field(wt)?,
            }
        }

        Ok(())
    }

    fn parse_single_node(
        data: &[u8],
        strings: &[&str],
        granularity: i64,
        lat_offset: i64,
        lon_offset: i64,
    ) -> Result<Option<RawOsmNode>, OsmIngestError> {
        let mut wire = ProtoWireReader::new(data);
        let mut id: u64 = 0;
        let mut lat_raw: i64 = 0;
        let mut lon_raw: i64 = 0;
        let mut keys = Vec::new();
        let mut vals = Vec::new();

        while let Some((field, wire_type)) = wire.next_tag()? {
            match (field, wire_type) {
                (1, ProtoWireType::Varint) => id = wire.read_sint64()? as u64,
                (2, ProtoWireType::LengthDelimited) => keys = wire.read_packed_uint32()?,
                (2, ProtoWireType::Varint) => keys.push(wire.read_uint32()?),
                (3, ProtoWireType::LengthDelimited) => vals = wire.read_packed_uint32()?,
                (3, ProtoWireType::Varint) => vals.push(wire.read_uint32()?),
                (8, ProtoWireType::Varint) => lat_raw = wire.read_sint64()?,
                (9, ProtoWireType::Varint) => lon_raw = wire.read_sint64()?,
                (_, wt) => wire.skip_field(wt)?,
            }
        }

        let lat = 1e-9 * ((lat_offset + (granularity * lat_raw)) as f64);
        let lon = 1e-9 * ((lon_offset + (granularity * lon_raw)) as f64);

        let mut tags = HashMap::new();
        for (&k, &v) in keys.iter().zip(vals.iter()) {
            if let (Some(&k_str), Some(&v_str)) = (strings.get(k as usize), strings.get(v as usize)) {
                tags.insert(k_str.to_string(), v_str.to_string());
            }
        }

        let elevation_m = tags.get("ele").and_then(|s| s.parse::<f64>().ok()).map(|e| e as i16);
        Ok(Some(RawOsmNode { id, lat, lon, elevation_m, tags }))
    }

    fn parse_dense_nodes(
        data: &[u8],
        strings: &[&str],
        granularity: i64,
        lat_offset: i64,
        lon_offset: i64,
        out_nodes: &mut Vec<RawOsmNode>,
    ) -> Result<(), OsmIngestError> {
        let mut wire = ProtoWireReader::new(data);
        let mut packed_ids = Vec::new();
        let mut packed_lats = Vec::new();
        let mut packed_lons = Vec::new();
        let mut keys_vals = Vec::new();

        while let Some((field, wire_type)) = wire.next_tag()? {
            match (field, wire_type) {
                (1, ProtoWireType::LengthDelimited) => packed_ids = wire.read_packed_sint64()?,
                (9, ProtoWireType::LengthDelimited) => packed_lats = wire.read_packed_sint64()?,
                (10, ProtoWireType::LengthDelimited) => packed_lons = wire.read_packed_sint64()?,
                (11, ProtoWireType::LengthDelimited) => keys_vals = wire.read_packed_int32()?,
                (_, wt) => wire.skip_field(wt)?,
            }
        }

        let count = packed_ids.len();
        if packed_lats.len() < count || packed_lons.len() < count {
            return Ok(());
        }

        out_nodes.reserve(count);

        let mut id_acc: i64 = 0;
        let mut lat_acc: i64 = 0;
        let mut lon_acc: i64 = 0;
        let mut kv_idx = 0;

        for i in 0..count {
            id_acc += packed_ids[i];
            lat_acc += packed_lats[i];
            lon_acc += packed_lons[i];

            let lat = 1e-9 * ((lat_offset + (granularity * lat_acc)) as f64);
            let lon = 1e-9 * ((lon_offset + (granularity * lon_acc)) as f64);

            let mut tags = HashMap::new();
            while kv_idx < keys_vals.len() {
                let k_tag = keys_vals[kv_idx];
                kv_idx += 1;
                if k_tag == 0 {
                    break;
                }
                if kv_idx < keys_vals.len() {
                    let v_tag = keys_vals[kv_idx];
                    kv_idx += 1;
                    if let (Some(&k_str), Some(&v_str)) = (strings.get(k_tag as usize), strings.get(v_tag as usize)) {
                        tags.insert(k_str.to_string(), v_str.to_string());
                    }
                }
            }

            let elevation_m = tags.get("ele").and_then(|s| s.parse::<f64>().ok()).map(|e| e as i16);
            out_nodes.push(RawOsmNode {
                id: id_acc as u64,
                lat,
                lon,
                elevation_m,
                tags,
            });
        }

        Ok(())
    }

    fn parse_way(data: &[u8], strings: &[&str]) -> Result<Option<RawOsmWay>, OsmIngestError> {
        let mut wire = ProtoWireReader::new(data);
        let mut id: u64 = 0;
        let mut keys = Vec::new();
        let mut vals = Vec::new();
        let mut refs_deltas = Vec::new();

        while let Some((field, wire_type)) = wire.next_tag()? {
            match (field, wire_type) {
                (1, ProtoWireType::Varint) => id = wire.read_int32()? as u64,
                (2, ProtoWireType::LengthDelimited) => keys = wire.read_packed_uint32()?,
                (2, ProtoWireType::Varint) => keys.push(wire.read_uint32()?),
                (3, ProtoWireType::LengthDelimited) => vals = wire.read_packed_uint32()?,
                (3, ProtoWireType::Varint) => vals.push(wire.read_uint32()?),
                (8, ProtoWireType::LengthDelimited) => refs_deltas = wire.read_packed_sint64()?,
                (8, ProtoWireType::Varint) => refs_deltas.push(wire.read_sint64()?),
                (_, wt) => wire.skip_field(wt)?,
            }
        }

        let mut node_refs = Vec::with_capacity(refs_deltas.len());
        let mut ref_acc: i64 = 0;
        for delta in refs_deltas {
            ref_acc += delta;
            node_refs.push(ref_acc as u64);
        }

        let mut tags = HashMap::new();
        for (&k, &v) in keys.iter().zip(vals.iter()) {
            if let (Some(&k_str), Some(&v_str)) = (strings.get(k as usize), strings.get(v as usize)) {
                tags.insert(k_str.to_string(), v_str.to_string());
            }
        }

        Ok(Some(RawOsmWay { id, node_refs, tags }))
    }

    fn parse_relation(data: &[u8], strings: &[&str]) -> Result<Option<RawOsmRelation>, OsmIngestError> {
        let mut wire = ProtoWireReader::new(data);
        let mut id: u64 = 0;
        let mut keys = Vec::new();
        let mut vals = Vec::new();
        let mut roles_sid = Vec::new();
        let mut memids_deltas = Vec::new();
        let mut types = Vec::new();

        while let Some((field, wire_type)) = wire.next_tag()? {
            match (field, wire_type) {
                (1, ProtoWireType::Varint) => id = wire.read_int32()? as u64,
                (2, ProtoWireType::LengthDelimited) => keys = wire.read_packed_uint32()?,
                (2, ProtoWireType::Varint) => keys.push(wire.read_uint32()?),
                (3, ProtoWireType::LengthDelimited) => vals = wire.read_packed_uint32()?,
                (3, ProtoWireType::Varint) => vals.push(wire.read_uint32()?),
                (8, ProtoWireType::LengthDelimited) => roles_sid = wire.read_packed_int32()?,
                (8, ProtoWireType::Varint) => roles_sid.push(wire.read_int32()?),
                (9, ProtoWireType::LengthDelimited) => memids_deltas = wire.read_packed_sint64()?,
                (9, ProtoWireType::Varint) => memids_deltas.push(wire.read_sint64()?),
                (10, ProtoWireType::LengthDelimited) => types = wire.read_packed_int32()?,
                (10, ProtoWireType::Varint) => types.push(wire.read_int32()?),
                (_, wt) => wire.skip_field(wt)?,
            }
        }

        let mut members = Vec::with_capacity(memids_deltas.len());
        let mut mem_acc: i64 = 0;
        for i in 0..memids_deltas.len() {
            mem_acc += memids_deltas[i];
            let role = if let Some(&sid) = roles_sid.get(i) {
                strings.get(sid as usize).copied().unwrap_or("").to_string()
            } else {
                String::new()
            };
            let member_type = match types.get(i).copied().unwrap_or(0) {
                0 => "node",
                1 => "way",
                _ => "relation",
            }
            .to_string();

            members.push(crate::osm_ingest::RawOsmMember {
                member_type,
                role,
                ref_id: mem_acc as u64,
            });
        }

        let mut tags = HashMap::new();
        for (&k, &v) in keys.iter().zip(vals.iter()) {
            if let (Some(&k_str), Some(&v_str)) = (strings.get(k as usize), strings.get(v as usize)) {
                tags.insert(k_str.to_string(), v_str.to_string());
            }
        }

        Ok(Some(RawOsmRelation { id, members, tags }))
    }
}
