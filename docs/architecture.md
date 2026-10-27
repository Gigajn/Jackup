# Jackup architecture

## Design goals

Jackup is organized around deep modules: callers depend on small interfaces,
while provider-specific complexity stays behind adapters. The control plane
(CLI, desktop, and web) is separate from the headless target agent.

## Dependency direction

```text
jackup-app ─────┐
jackup-cli ─────┼──> jackup-orchestrator ──> jackup-protocol
jackup-agent ───┘             │                    │
                              v                    v
                       jackup-storage       jackup-transport
                              │
                              v
                         jackup-crypto

All modules may use jackup-core and jackup-config.
```

`jackup-core` contains domain types only. It must not depend on operating
systems, network clients, databases, or UI frameworks. Infrastructure adapters
implement interfaces owned by the module that needs them; this keeps the seam
stable and makes in-memory implementations possible for tests.

## Crate responsibilities

| Crate | Responsibility | Must not contain |
| --- | --- | --- |
| `jackup-core` | Domain models and invariants | I/O or provider SDKs |
| `jackup-config` | Validated configuration and defaults | Secrets in plain text |
| `jackup-crypto` | Key derivation and authenticated encryption | Vault persistence |
| `jackup-protocol` | Versioned control messages | Transport implementation |
| `jackup-transport` | mTLS/QUIC/HTTP adapters | Business workflows |
| `jackup-storage` | SQLite and filesystem adapters | UI or orchestration policy |
| `jackup-backup` | Backup pipeline and storage adapters | Credential management |
| `jackup-orchestrator` | Use cases and provider ports | CLI/UI concerns |
| `jackup-agent` | Target-server composition root | Control-plane UI |
| `jackup-cli` | CLI adapter and composition root | Domain rules |
| `jackup-app` | Desktop/web presentation adapter | Provider-specific logic |

## Composition roots

There are three composition roots: `jackup-cli`, `jackup-agent`, and
`jackup-app`. Dependencies are constructed there and passed into use-case
modules. Domain and application modules do not construct global clients,
read environment variables, or perform process-wide initialization.

## Planned implementation order

1. Configuration and protocol validation.
2. Agent pairing and transport.
3. Persistent state and audit log.
4. Deployment and backup orchestration.
5. Migration providers.
6. Desktop/web presentation.

