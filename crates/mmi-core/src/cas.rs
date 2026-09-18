//! Content-Addressed Storage (CAS) for immutable blobs.

use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use sha2::{Digest, Sha256};

use crate::error::CoreError;

/// Content-Addressed Store indexing data blobs by their BLAKE3 digest.
#[derive(Debug, Clone)]
pub struct ContentAddressedStore {
    objects_dir: PathBuf,
    tmp_dir: PathBuf,
}

impl ContentAddressedStore {
    /// Initializes a CAS at the specified directory (e.g. `.mmistudio/`).
    pub fn new(base_dir: impl AsRef<Path>) -> Result<Self, CoreError> {
        let base = base_dir.as_ref();
        let objects_dir = base.join("objects");
        let tmp_dir = base.join("tmp");

        fs::create_dir_all(&objects_dir)?;
        fs::create_dir_all(&tmp_dir)?;

        Ok(Self {
            objects_dir,
            tmp_dir,
        })
    }

    /// Returns the objects directory path.
    pub fn objects_dir(&self) -> &Path {
        &self.objects_dir
    }

    /// Checks if a blob with the given BLAKE3 hash already exists in storage.
    pub fn has_blob(&self, blake3_hex: &str) -> bool {
        self.objects_dir.join(blake3_hex).exists()
    }

    /// Retrieves the path to a stored blob.
    pub fn blob_path(&self, blake3_hex: &str) -> Result<PathBuf, CoreError> {
        let path = self.objects_dir.join(blake3_hex);
        if !path.exists() {
            return Err(CoreError::NotFound(format!("Blob '{}' not in CAS", blake3_hex)));
        }
        Ok(path)
    }

    /// Opens a stored blob for reading.
    pub fn get_reader(&self, blake3_hex: &str) -> Result<File, CoreError> {
        let path = self.blob_path(blake3_hex)?;
        let file = OpenOptions::new().read(true).open(path)?;
        Ok(file)
    }

    /// Reads an entire blob into memory.
    pub fn read_bytes(&self, blake3_hex: &str) -> Result<Vec<u8>, CoreError> {
        let mut file = self.get_reader(blake3_hex)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        Ok(buf)
    }

    /// Ingests in-memory data, computing BLAKE3 and SHA-256 digests, and writes
    /// atomically into `.mmistudio/objects/<blake3_hex>`.
    pub fn put_bytes(&self, data: &[u8]) -> Result<(String, String), CoreError> {
        let blake3_hex = blake3::hash(data).to_hex().to_string();
        let mut sha256_hasher = Sha256::new();
        sha256_hasher.update(data);
        let sha256_hex = hex::encode(sha256_hasher.finalize());

        let target_path = self.objects_dir.join(&blake3_hex);
        if target_path.exists() {
            // Already present, deduplicated
            return Ok((blake3_hex, sha256_hex));
        }

        // Write to temporary file first
        let tmp_file_path = self.tmp_dir.join(format!("{}.tmp", blake3_hex));
        {
            let mut f = File::create(&tmp_file_path)?;
            f.write_all(data)?;
            f.sync_all()?;
        }

        // Atomically rename into place
        fs::rename(&tmp_file_path, &target_path)?;

        // Make blob read-only
        let mut perms = fs::metadata(&target_path)?.permissions();
        perms.set_readonly(true);
        let _ = fs::set_permissions(&target_path, perms);

        Ok((blake3_hex, sha256_hex))
    }

    /// Ingests a file from disk into the CAS using streaming reads.
    pub fn put_file(&self, source_path: impl AsRef<Path>) -> Result<(String, String), CoreError> {
        let mut file = File::open(source_path)?;
        let mut blake3_hasher = blake3::Hasher::new();
        let mut sha256_hasher = Sha256::new();

        let tmp_file_path = self.tmp_dir.join(format!("stream_{}.tmp", std::process::id()));
        let mut tmp_file = File::create(&tmp_file_path)?;

        let mut buffer = [0u8; 64 * 1024];
        loop {
            let n = file.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            tmp_file.write_all(&buffer[..n])?;
            blake3_hasher.update(&buffer[..n]);
            sha256_hasher.update(&buffer[..n]);
        }
        tmp_file.sync_all()?;
        drop(tmp_file);

        let blake3_hex = blake3_hasher.finalize().to_hex().to_string();
        let sha256_hex = hex::encode(sha256_hasher.finalize());

        let target_path = self.objects_dir.join(&blake3_hex);
        if target_path.exists() {
            let _ = fs::remove_file(&tmp_file_path);
            return Ok((blake3_hex, sha256_hex));
        }

        fs::rename(&tmp_file_path, &target_path)?;

        // Set read-only permissions
        let mut perms = fs::metadata(&target_path)?.permissions();
        perms.set_readonly(true);
        let _ = fs::set_permissions(&target_path, perms);

        Ok((blake3_hex, sha256_hex))
    }

    /// Materializes a stored blob into a destination path using fast reflink/clonefile,
    /// falling back to hardlink and then standard copy.
    pub fn materialize(&self, blake3_hex: &str, destination: impl AsRef<Path>) -> Result<(), CoreError> {
        let src = self.blob_path(blake3_hex)?;
        let dst = destination.as_ref();

        if let Some(parent) = dst.parent() {
            fs::create_dir_all(parent)?;
        }

        if dst.exists() {
            fs::remove_file(dst)?;
        }

        // Try clonefile on macOS
        #[cfg(target_os = "macos")]
        {
            use std::ffi::CString;
            use std::os::unix::ffi::OsStrExt;

            let src_c = CString::new(src.as_os_str().as_bytes()).map_err(|e| {
                CoreError::ReflinkFailed(format!("Invalid CString for source: {}", e))
            })?;
            let dst_c = CString::new(dst.as_os_str().as_bytes()).map_err(|e| {
                CoreError::ReflinkFailed(format!("Invalid CString for destination: {}", e))
            })?;

            let ret = unsafe { libc::clonefile(src_c.as_ptr(), dst_c.as_ptr(), 0) };
            if ret == 0 {
                return Ok(());
            }
        }

        // Fallback 1: Hard link (read-only)
        if fs::hard_link(&src, dst).is_ok() {
            return Ok(());
        }

        // Fallback 2: Regular file copy
        fs::copy(&src, dst)?;
        Ok(())
    }
}
