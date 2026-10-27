//! Versioned messages exchanged between control planes and agents.

use jackup_core::ServerSummary;
use serde::{Deserialize, Serialize};

pub const PROTOCOL_VERSION: u16 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Request {
    GetServerSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Response {
    ServerSummary(ServerSummary),
}
