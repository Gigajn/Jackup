use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Server {
    pub id: String,
    pub name: String,
    pub endpoint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ServerStatus {
    Unpaired,
    Paired,
    Unreachable,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ServerSummary {
    pub server: Server,
    pub status: ServerStatus,
}

impl Server {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        endpoint: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            endpoint: endpoint.into(),
        }
    }
}
