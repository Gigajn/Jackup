//! Application use cases. This crate owns policy and depends on ports, not
//! concrete network, database, or UI implementations.

use jackup_core::ServerSummary;
use jackup_storage::{ServerRepository, StorageError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrchestrationError {
    #[error(transparent)]
    Storage(#[from] StorageError),
}

/// Lists servers through the storage port.
///
/// # Errors
///
/// Returns [`OrchestrationError`] when the storage port fails.
pub fn list_servers(
    repository: &impl ServerRepository,
) -> Result<Vec<ServerSummary>, OrchestrationError> {
    repository.list().map_err(Into::into)
}
