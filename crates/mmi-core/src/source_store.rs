//! SourceStore: The sole authorized access point to the immutable evidence repository.

use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use sha2::{Digest, Sha256};

use crate::error::CoreError;
use crate::immutable_path::ImmutablePath;

/// `SourceStore` is the architectural gatekeeper to `originals/`.
/// It provides strictly read-only access to files within the evidence repository
/// and prevents path traversals or unauthorized write handles.
#[derive(Debug, Clone)]
pub struct SourceStore {
    root: ImmutablePath,
}

impl SourceStore {
    /// Creates a new `SourceStore` rooted at the specified directory.
    pub fn new(root_dir: impl AsRef<Path>) -> Result<Self, CoreError> {
        let p = root_dir.as_ref();
        if !p.exists() {
            return Err(CoreError::NotFound(format!("Source root does not exist: {}", p.display())));
        }
        if !p.is_dir() {
            return Err(CoreError::ImmutabilityViolation(format!(
                "Source root is not a directory: {}",
                p.display()
            )));
        }

        let canonical = p.canonicalize().map_err(CoreError::Io)?;
        Ok(Self {
            root: ImmutablePath::new_unchecked(canonical),
        })
    }

    /// Returns the canonical root path of the store.
    pub fn root(&self) -> &ImmutablePath {
        &self.root
    }

    /// Safely resolves a relative path within the store, ensuring it does not
    /// escape the immutable root boundary.
    pub fn resolve(&self, relative_path: impl AsRef<Path>) -> Result<ImmutablePath, CoreError> {
        let rel = relative_path.as_ref();
        let candidate = self.root.as_path().join(rel);

        // Prevent non-existent resolution if canonicalization fails
        let canonical = match candidate.canonicalize() {
            Ok(c) => c,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Err(CoreError::NotFound(format!(
                    "Item '{}' not found under source root",
                    rel.display()
                )));
            }
            Err(e) => return Err(CoreError::Io(e)),
        };

        if !canonical.starts_with(self.root.as_path()) {
            return Err(CoreError::PathEscape(format!(
                "Path '{}' escapes immutable root '{}'",
                rel.display(),
                self.root.display()
            )));
        }

        Ok(ImmutablePath::new_unchecked(canonical))
    }

    /// Opens a file strictly for reading. Write access is never permitted.
    pub fn open_read(&self, relative_path: impl AsRef<Path>) -> Result<File, CoreError> {
        let resolved = self.resolve(relative_path)?;
        if !resolved.is_file() {
            return Err(CoreError::ImmutabilityViolation(format!(
                "Target '{}' is not a regular file",
                resolved.display()
            )));
        }

        let file = OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .truncate(false)
            .open(&resolved)?;

        Ok(file)
    }

    /// Reads all bytes of the target file into memory.
    pub fn read_bytes(&self, relative_path: impl AsRef<Path>) -> Result<Vec<u8>, CoreError> {
        let mut file = self.open_read(relative_path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        Ok(buf)
    }

    /// Computes dual hashes (BLAKE3 and SHA-256) in a single streaming pass.
    pub fn stream_hash(&self, relative_path: impl AsRef<Path>) -> Result<(String, String), CoreError> {
        let mut file = self.open_read(relative_path)?;
        let mut blake3_hasher = blake3::Hasher::new();
        let mut sha256_hasher = Sha256::new();

        let mut buffer = [0u8; 64 * 1024];
        loop {
            let n = file.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            blake3_hasher.update(&buffer[..n]);
            sha256_hasher.update(&buffer[..n]);
        }

        let blake3_hex = blake3_hasher.finalize().to_hex().to_string();
        let sha256_hex = hex::encode(sha256_hasher.finalize());

        Ok((blake3_hex, sha256_hex))
    }

    /// Asserts that write operations are rejected and the directory remains intact.
    pub fn assert_immutability(&self) -> Result<(), CoreError> {
        let test_file = self.root.as_path().join(".immutability_probe_temp");
        // Verify that OpenOptions with write=true is refused or test probe fails
        match OpenOptions::new().write(true).create_new(true).open(&test_file) {
            Ok(mut f) => {
                // If it succeeded, remove it immediately and report violation
                let _ = f.write_all(b"probe");
                let _ = std::fs::remove_file(&test_file);
                Err(CoreError::ImmutabilityViolation(
                    "SourceStore root allows write access; permissions check failed".to_string(),
                ))
            }
            Err(_) => {
                // Expected behavior if directory is read-only, or verification passed
                Ok(())
            }
        }
    }
}
