//! Asset cataloger linking extracted stage files to MMIProject descriptors and CAS thumbnails.

use mmi_core::{
    project::AssetDescriptor, ContentAddressedStore, CoreError, StageEntry,
};

use crate::decoder::AssetDecoder;
use crate::thumbnail::ThumbnailGenerator;

pub struct AssetCataloger<'a> {
    cas: &'a ContentAddressedStore,
}

impl<'a> AssetCataloger<'a> {
    pub fn new(cas: &'a ContentAddressedStore) -> Self {
        Self { cas }
    }

    /// Inspects an extracted stage entry, checks if it is a visual asset, decodes it,
    /// generates a thumbnail in CAS, and constructs an `AssetDescriptor`.
    pub fn catalog_entry(&self, entry: &StageEntry) -> Result<Option<AssetDescriptor>, CoreError> {
        let is_visual = entry.logical_path.ends_with(".precomp")
            || entry.logical_path.ends_with(".png")
            || entry.logical_path.ends_with(".bmp");

        if !is_visual {
            return Ok(None);
        }

        let bytes = self.cas.read_bytes(&entry.blob_id)?;
        if let Ok(decoded) = AssetDecoder::decode(&bytes) {
            let thumb_blob_id = ThumbnailGenerator::create_thumbnail(&decoded, 64, self.cas)?;

            let asset_id = entry
                .logical_path
                .replace('/', "_")
                .replace('.', "_");

            Ok(Some(AssetDescriptor {
                asset_id,
                logical_path: entry.logical_path.clone(),
                module: entry.module_name.clone().unwrap_or_else(|| "root".to_string()),
                blob_id: thumb_blob_id,
                format: decoded.source_format,
                width: Some(decoded.width),
                height: Some(decoded.height),
                byte_size: entry.byte_size,
                can_edit: !entry.is_signed,
                can_rebuild: !entry.is_signed,
            }))
        } else {
            Ok(None)
        }
    }
}
