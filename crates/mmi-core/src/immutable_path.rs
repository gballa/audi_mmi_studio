//! Immutable path wrapper guaranteeing read-only access.

use std::fmt;
use std::ops::Deref;
use std::path::{Path, PathBuf};

/// `ImmutablePath` wraps a `PathBuf` representing a verified path within the
/// immutable evidence corpus (`originals/`). It prevents accidental conversion
/// to mutable paths and exposes only read-only query methods.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImmutablePath(pub(crate) PathBuf);

impl ImmutablePath {
    /// Internal constructor used exclusively by `SourceStore`.
    pub(crate) fn new_unchecked(path: PathBuf) -> Self {
        Self(path)
    }

    /// Returns a reference to the underlying `Path`.
    pub fn as_path(&self) -> &Path {
        &self.0
    }

    /// Checks if the target path exists.
    pub fn exists(&self) -> bool {
        self.0.exists()
    }

    /// Checks if the path points to a regular file.
    pub fn is_file(&self) -> bool {
        self.0.is_file()
    }

    /// Checks if the path points to a directory.
    pub fn is_dir(&self) -> bool {
        self.0.is_dir()
    }

    /// Returns the file metadata.
    pub fn metadata(&self) -> std::io::Result<std::fs::Metadata> {
        self.0.metadata()
    }

    /// Returns the file extension if available.
    pub fn extension(&self) -> Option<&std::ffi::OsStr> {
        self.0.extension()
    }

    /// Returns the file name if available.
    pub fn file_name(&self) -> Option<&std::ffi::OsStr> {
        self.0.file_name()
    }
}

impl Deref for ImmutablePath {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<Path> for ImmutablePath {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl fmt::Debug for ImmutablePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ImmutablePath({:?})", self.0)
    }
}

impl fmt::Display for ImmutablePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.display())
    }
}
