//! Validated configuration types shared by the composition roots.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AgentConfig {
    pub listen_address: String,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            listen_address: "127.0.0.1:8787".to_owned(),
        }
    }
}
