//! Transport adapters. Network implementation is intentionally kept behind
//! this crate's small interface.

use jackup_protocol::{Request, Response};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransportError {
    #[error("transport is not configured")]
    NotConfigured,
}

pub trait ControlChannel {
    /// Sends one versioned request and returns its response.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError`] when the channel has not been configured.
    fn request(&self, request: Request) -> Result<Response, TransportError>;
}
