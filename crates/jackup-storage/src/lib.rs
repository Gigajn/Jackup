//! Persistence adapters for configuration, state, and audit records.

use jackup_core::ServerSummary;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("storage adapter is not configured")]
    NotConfigured,
}

pub trait ServerRepository {
    /// Lists all servers known to the local control plane.
    ///
    /// # Errors
    ///
    /// Returns [`StorageError`] when the persistence adapter is unavailable.
    fn list(&self) -> Result<Vec<ServerSummary>, StorageError>;
}
