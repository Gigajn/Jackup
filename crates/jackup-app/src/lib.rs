use jackup_core::ServerSummary;

#[derive(Debug, Default)]
pub struct AppState {
    servers: Vec<ServerSummary>,
}

impl AppState {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn servers(&self) -> &[ServerSummary] {
        &self.servers
    }

    pub fn add_server(&mut self, server: ServerSummary) {
        self.servers.push(server);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jackup_core::{Server, ServerStatus};

    #[test]
    fn stores_server_summaries_for_any_ui_target() {
        let mut state = AppState::new();
        state.add_server(ServerSummary {
            server: Server::new("local", "Local server", "127.0.0.1"),
            status: ServerStatus::Unpaired,
        });

        assert_eq!(state.servers().len(), 1);
        assert_eq!(state.servers()[0].server.id, "local");
    }
}
