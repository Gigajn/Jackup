//! Backup pipeline interfaces. Compression, encryption, and remote storage
//! adapters will be implemented behind these interfaces.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BackupError {
    #[error("backup adapter is not configured")]
    NotConfigured,
}

pub trait BackupStore {
    /// Stores one already-prepared backup payload.
    ///
    /// # Errors
    ///
    /// Returns [`BackupError`] when the backup adapter is unavailable.
    fn put(&self, name: &str, data: &[u8]) -> Result<(), BackupError>;
}
