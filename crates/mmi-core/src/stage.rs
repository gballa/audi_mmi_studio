//! StageStore: SQLite-backed stage database tracking extracted file trees mapped to CAS blobs.

use std::path::{Path, PathBuf};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

use crate::error::CoreError;

/// An entry in the staging filesystem database.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StageEntry {
    pub logical_path: String,
    pub blob_id: String,
    pub sha256_hex: String,
    pub byte_size: u64,
    pub is_signed: bool,
    pub module_name: Option<String>,
}

/// Stage database managing workspace stages in `.mmistudio/stages/`.
pub struct StageStore {
    conn: Connection,
    stage_name: String,
    db_path: PathBuf,
}

impl StageStore {
    /// Creates or opens a named stage database under `.mmistudio/stages/<stage_name>.sqlite`.
    pub fn open(stages_dir: impl AsRef<Path>, stage_name: impl Into<String>) -> Result<Self, CoreError> {
        let name = stage_name.into();
        let dir = stages_dir.as_ref();
        std::fs::create_dir_all(dir)?;
        let db_path = dir.join(format!("{}.sqlite", name));

        let conn = Connection::open(&db_path)?;
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             CREATE TABLE IF NOT EXISTS stage_entries (
                 logical_path TEXT PRIMARY KEY,
                 blob_id TEXT NOT NULL,
                 sha256_hex TEXT NOT NULL,
                 byte_size INTEGER NOT NULL,
                 is_signed INTEGER NOT NULL,
                 module_name TEXT
             );
             CREATE INDEX IF NOT EXISTS idx_stage_module ON stage_entries(module_name);
             CREATE INDEX IF NOT EXISTS idx_stage_blob ON stage_entries(blob_id);",
        )?;

        Ok(Self {
            conn,
            stage_name: name,
            db_path,
        })
    }

    /// Creates an in-memory stage database for tests.
    pub fn in_memory(stage_name: impl Into<String>) -> Result<Self, CoreError> {
        let name = stage_name.into();
        let conn = Connection::open_in_memory()?;
        conn.execute_batch(
            "CREATE TABLE stage_entries (
                 logical_path TEXT PRIMARY KEY,
                 blob_id TEXT NOT NULL,
                 sha256_hex TEXT NOT NULL,
                 byte_size INTEGER NOT NULL,
                 is_signed INTEGER NOT NULL,
                 module_name TEXT
             );",
        )?;

        Ok(Self {
            conn,
            stage_name: name,
            db_path: PathBuf::from(":memory:"),
        })
    }

    /// Records an extracted file in the stage.
    pub fn put_entry(&mut self, entry: &StageEntry) -> Result<(), CoreError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO stage_entries 
             (logical_path, blob_id, sha256_hex, byte_size, is_signed, module_name)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                entry.logical_path,
                entry.blob_id,
                entry.sha256_hex,
                entry.byte_size,
                if entry.is_signed { 1 } else { 0 },
                entry.module_name,
            ],
        )?;
        Ok(())
    }

    /// Retrieves an entry by its logical path.
    pub fn get_entry(&self, logical_path: &str) -> Result<Option<StageEntry>, CoreError> {
        let mut stmt = self.conn.prepare(
            "SELECT logical_path, blob_id, sha256_hex, byte_size, is_signed, module_name
             FROM stage_entries WHERE logical_path = ?1",
        )?;

        let mut rows = stmt.query(params![logical_path])?;
        if let Some(row) = rows.next()? {
            let is_signed_int: i64 = row.get(4)?;
            Ok(Some(StageEntry {
                logical_path: row.get(0)?,
                blob_id: row.get(1)?,
                sha256_hex: row.get(2)?,
                byte_size: row.get(3)?,
                is_signed: is_signed_int != 0,
                module_name: row.get(5)?,
            }))
        } else {
            Ok(None)
        }
    }

    /// Counts total registered entries in the stage.
    pub fn count_entries(&self) -> Result<usize, CoreError> {
        let count: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM stage_entries", [], |row| row.get(0))?;
        Ok(count as usize)
    }

    /// Lists all entries registered in the stage.
    pub fn list_entries(&self) -> Result<Vec<StageEntry>, CoreError> {
        let mut stmt = self.conn.prepare(
            "SELECT logical_path, blob_id, sha256_hex, byte_size, is_signed, module_name
             FROM stage_entries ORDER BY logical_path ASC",
        )?;

        let rows = stmt.query_map([], |row| {
            let is_signed_int: i64 = row.get(4)?;
            Ok(StageEntry {
                logical_path: row.get(0)?,
                blob_id: row.get(1)?,
                sha256_hex: row.get(2)?,
                byte_size: row.get(3)?,
                is_signed: is_signed_int != 0,
                module_name: row.get(5)?,
            })
        })?;

        let mut entries = Vec::new();
        for r in rows {
            entries.push(r?);
        }
        Ok(entries)
    }

    pub fn stage_name(&self) -> &str {
        &self.stage_name
    }

    pub fn db_path(&self) -> &Path {
        &self.db_path
    }
}
